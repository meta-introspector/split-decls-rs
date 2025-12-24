use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'db> Type<'db> {
    pub(crate) fn new_with_resolver(
        db: &'db dyn HirDatabase,
        resolver: &Resolver<'_>,
        ty: Ty<'db>,
    ) -> Self {
        Type::new_with_resolver_inner(db, resolver, ty)
    }
    pub(crate) fn new_with_resolver_inner(
        db: &'db dyn HirDatabase,
        resolver: &Resolver<'_>,
        ty: Ty<'db>,
    ) -> Self {
        let environment = resolver
            .generic_def()
            .map_or_else(
                || TraitEnvironment::empty(resolver.krate()),
                |d| db.trait_environment(d),
            );
        Type { env: environment, ty }
    }
    pub(crate) fn new_for_crate(krate: base_db::Crate, ty: Ty<'db>) -> Self {
        Type {
            env: TraitEnvironment::empty(krate),
            ty,
        }
    }
    fn new(
        db: &'db dyn HirDatabase,
        lexical_env: impl HasResolver,
        ty: Ty<'db>,
    ) -> Self {
        let resolver = lexical_env.resolver(db);
        let environment = resolver
            .generic_def()
            .map_or_else(
                || TraitEnvironment::empty(resolver.krate()),
                |d| db.trait_environment(d),
            );
        Type { env: environment, ty }
    }
    fn from_def(
        db: &'db dyn HirDatabase,
        def: impl Into<TyDefId> + HasResolver,
    ) -> Self {
        let interner = DbInterner::new_with(db, None, None);
        let ty = db.ty(def.into());
        let def = match def.into() {
            TyDefId::AdtId(it) => GenericDefId::AdtId(it),
            TyDefId::TypeAliasId(it) => GenericDefId::TypeAliasId(it),
            TyDefId::BuiltinType(_) => {
                return Type::new(db, def, ty.skip_binder());
            }
        };
        let args = GenericArgs::error_for_item(interner, def.into());
        Type::new(db, def, ty.instantiate(interner, args))
    }
    fn from_def_params(
        db: &'db dyn HirDatabase,
        def: impl Into<TyDefId> + HasResolver,
    ) -> Self {
        let ty = db.ty(def.into());
        Type::new(db, def, ty.instantiate_identity())
    }
    fn from_value_def(
        db: &'db dyn HirDatabase,
        def: impl Into<ValueTyDefId> + HasResolver,
    ) -> Self {
        let interner = DbInterner::new_with(db, None, None);
        let Some(ty) = db.value_ty(def.into()) else {
            return Type::new(db, def, Ty::new_error(interner, ErrorGuaranteed));
        };
        let def = match def.into() {
            ValueTyDefId::ConstId(it) => GenericDefId::ConstId(it),
            ValueTyDefId::FunctionId(it) => GenericDefId::FunctionId(it),
            ValueTyDefId::StructId(it) => GenericDefId::AdtId(AdtId::StructId(it)),
            ValueTyDefId::UnionId(it) => GenericDefId::AdtId(AdtId::UnionId(it)),
            ValueTyDefId::EnumVariantId(it) => {
                GenericDefId::AdtId(AdtId::EnumId(it.lookup(db).parent))
            }
            ValueTyDefId::StaticId(_) => {
                return Type::new(db, def, ty.skip_binder());
            }
        };
        let args = GenericArgs::error_for_item(interner, def.into());
        Type::new(db, def, ty.instantiate(interner, args))
    }
    pub fn new_slice(ty: Self) -> Self {
        let interner = DbInterner::conjure();
        Type {
            env: ty.env,
            ty: Ty::new_slice(interner, ty.ty),
        }
    }
    pub fn new_tuple(krate: base_db::Crate, tys: &[Self]) -> Self {
        let tys = tys.iter().map(|it| it.ty);
        let interner = DbInterner::conjure();
        Type {
            env: TraitEnvironment::empty(krate),
            ty: Ty::new_tup_from_iter(interner, tys),
        }
    }
    pub fn is_unit(&self) -> bool {
        self.ty.is_unit()
    }
    pub fn is_bool(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Bool)
    }
    pub fn is_str(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Str)
    }
    pub fn is_never(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Never)
    }
    pub fn is_mutable_reference(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Ref(.., hir_ty::next_solver::Mutability::Mut))
    }
    pub fn is_reference(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Ref(..))
    }
    pub fn contains_reference(&self, db: &'db dyn HirDatabase) -> bool {
        let interner = DbInterner::new_with(db, None, None);
        return self.ty.visit_with(&mut Visitor { interner }).is_break();
        fn is_phantom_data(db: &dyn HirDatabase, adt_id: AdtId) -> bool {
            match adt_id {
                AdtId::StructId(s) => {
                    let flags = db.struct_signature(s).flags;
                    flags.contains(StructFlags::IS_PHANTOM_DATA)
                }
                AdtId::UnionId(_) | AdtId::EnumId(_) => false,
            }
        }
        struct Visitor<'db> {
            interner: DbInterner<'db>,
        }
        impl<'db> TypeVisitor<DbInterner<'db>> for Visitor<'db> {
            type Result = ControlFlow<()>;
            fn visit_ty(&mut self, ty: Ty<'db>) -> Self::Result {
                match ty.kind() {
                    TyKind::Ref(..) => ControlFlow::Break(()),
                    TyKind::Adt(
                        adt_def,
                        args,
                    ) if !is_phantom_data(self.interner.db(), adt_def.def_id().0) => {
                        let _variant_id_to_fields = |id: VariantId| {
                            let variant_data = &id.fields(self.interner.db());
                            if variant_data.fields().is_empty() {
                                vec![]
                            } else {
                                let field_types = self.interner.db().field_types(id);
                                variant_data
                                    .fields()
                                    .iter()
                                    .map(|(idx, _)| {
                                        field_types[idx].instantiate(self.interner, args)
                                    })
                                    .filter(|it| !it.references_non_lt_error())
                                    .collect()
                            }
                        };
                        let variant_id_to_fields = |_: VariantId| vec![];
                        let variants: Vec<Vec<Ty<'db>>> = match adt_def.def_id().0 {
                            AdtId::StructId(id) => vec![variant_id_to_fields(id.into())],
                            AdtId::EnumId(id) => {
                                id.enum_variants(self.interner.db())
                                    .variants
                                    .iter()
                                    .map(|&(variant_id, _, _)| variant_id_to_fields(
                                        variant_id.into(),
                                    ))
                                    .collect()
                            }
                            AdtId::UnionId(id) => vec![variant_id_to_fields(id.into())],
                        };
                        variants
                            .into_iter()
                            .flat_map(|variant| variant.into_iter())
                            .try_for_each(|ty| ty.visit_with(self))?;
                        args.visit_with(self)
                    }
                    _ => ty.super_visit_with(self),
                }
            }
        }
    }
    pub fn as_reference(&self) -> Option<(Type<'db>, Mutability)> {
        let TyKind::Ref(_lt, ty, m) = self.ty.kind() else { return None };
        let m = Mutability::from_mutable(
            matches!(m, hir_ty::next_solver::Mutability::Mut),
        );
        Some((self.derived(ty), m))
    }
    pub fn add_reference(&self, mutability: Mutability) -> Self {
        let interner = DbInterner::conjure();
        let ty_mutability = match mutability {
            Mutability::Shared => hir_ty::next_solver::Mutability::Not,
            Mutability::Mut => hir_ty::next_solver::Mutability::Mut,
        };
        self.derived(
            Ty::new_ref(interner, Region::error(interner), self.ty, ty_mutability),
        )
    }
    pub fn is_slice(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Slice(..))
    }
    pub fn is_usize(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Uint(rustc_type_ir::UintTy::Usize))
    }
    pub fn is_float(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Float(_))
    }
    pub fn is_char(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Char)
    }
    pub fn is_int_or_uint(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Int(_) | TyKind::Uint(_))
    }
    pub fn is_scalar(&self) -> bool {
        matches!(
            self.ty.kind(), TyKind::Bool | TyKind::Char | TyKind::Int(_) |
            TyKind::Uint(_) | TyKind::Float(_)
        )
    }
    pub fn is_tuple(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Tuple(..))
    }
    pub fn remove_ref(&self) -> Option<Type<'db>> {
        match self.ty.kind() {
            TyKind::Ref(_, ty, _) => Some(self.derived(ty)),
            _ => None,
        }
    }
    pub fn as_slice(&self) -> Option<Type<'db>> {
        match self.ty.kind() {
            TyKind::Slice(ty) => Some(self.derived(ty)),
            _ => None,
        }
    }
    pub fn strip_references(&self) -> Self {
        self.derived(self.ty.strip_references())
    }
    pub fn strip_reference(&self) -> Self {
        self.derived(self.ty.strip_reference())
    }
    pub fn is_unknown(&self) -> bool {
        self.ty.is_ty_error()
    }
    /// Checks that particular type `ty` implements `std::future::IntoFuture` or
    /// `std::future::Future` and returns the `Output` associated type.
    /// This function is used in `.await` syntax completion.
    pub fn into_future_output(&self, db: &'db dyn HirDatabase) -> Option<Type<'db>> {
        let trait_ = LangItem::IntoFutureIntoFuture
            .resolve_function(db, self.env.krate)
            .and_then(|into_future_fn| {
                let assoc_item = as_assoc_item(db, AssocItem::Function, into_future_fn)?;
                let into_future_trait = assoc_item.container_or_implemented_trait(db)?;
                Some(into_future_trait.id)
            })
            .or_else(|| LangItem::Future.resolve_trait(db, self.env.krate))?;
        if !traits::implements_trait_unique(self.ty, db, self.env.clone(), trait_) {
            return None;
        }
        let output_assoc_type = trait_
            .trait_items(db)
            .associated_type_by_name(&Name::new_symbol_root(sym::Output))?;
        self.normalize_trait_assoc_type(db, &[], output_assoc_type.into())
    }
    /// This does **not** resolve `IntoFuture`, only `Future`.
    pub fn future_output(self, db: &'db dyn HirDatabase) -> Option<Type<'db>> {
        let future_output = LangItem::FutureOutput
            .resolve_type_alias(db, self.env.krate)?;
        self.normalize_trait_assoc_type(db, &[], future_output.into())
    }
    /// This does **not** resolve `IntoIterator`, only `Iterator`.
    pub fn iterator_item(self, db: &'db dyn HirDatabase) -> Option<Type<'db>> {
        let iterator_trait = LangItem::Iterator.resolve_trait(db, self.env.krate)?;
        let iterator_item = iterator_trait
            .trait_items(db)
            .associated_type_by_name(&Name::new_symbol_root(sym::Item))?;
        self.normalize_trait_assoc_type(db, &[], iterator_item.into())
    }
    pub fn impls_iterator(self, db: &'db dyn HirDatabase) -> bool {
        let Some(iterator_trait) = LangItem::Iterator.resolve_trait(db, self.env.krate)
        else {
            return false;
        };
        traits::implements_trait_unique(self.ty, db, self.env.clone(), iterator_trait)
    }
    /// Resolves the projection `<Self as IntoIterator>::IntoIter` and returns the resulting type
    pub fn into_iterator_iter(self, db: &'db dyn HirDatabase) -> Option<Type<'db>> {
        let trait_ = LangItem::IntoIterIntoIter
            .resolve_function(db, self.env.krate)
            .and_then(|into_iter_fn| {
                let assoc_item = as_assoc_item(db, AssocItem::Function, into_iter_fn)?;
                let into_iter_trait = assoc_item.container_or_implemented_trait(db)?;
                Some(into_iter_trait.id)
            })?;
        if !traits::implements_trait_unique(self.ty, db, self.env.clone(), trait_) {
            return None;
        }
        let into_iter_assoc_type = trait_
            .trait_items(db)
            .associated_type_by_name(&Name::new_symbol_root(sym::IntoIter))?;
        self.normalize_trait_assoc_type(db, &[], into_iter_assoc_type.into())
    }
    /// Checks that particular type `ty` implements `std::ops::FnOnce`.
    ///
    /// This function can be used to check if a particular type is callable, since FnOnce is a
    /// supertrait of Fn and FnMut, so all callable types implements at least FnOnce.
    pub fn impls_fnonce(&self, db: &'db dyn HirDatabase) -> bool {
        let fnonce_trait = match FnTrait::FnOnce.get_id(db, self.env.krate) {
            Some(it) => it,
            None => return false,
        };
        traits::implements_trait_unique(self.ty, db, self.env.clone(), fnonce_trait)
    }
    pub fn impls_trait(
        &self,
        db: &'db dyn HirDatabase,
        trait_: Trait,
        args: &[Type<'db>],
    ) -> bool {
        let interner = DbInterner::new_with(db, None, None);
        let args = generic_args_from_tys(
            interner,
            trait_.id.into(),
            std::iter::once(self.ty).chain(args.iter().map(|ty| ty.ty)),
        );
        traits::implements_trait_unique_with_args(db, self.env.clone(), trait_.id, args)
    }
    pub fn normalize_trait_assoc_type(
        &self,
        db: &'db dyn HirDatabase,
        args: &[Type<'db>],
        alias: TypeAlias,
    ) -> Option<Type<'db>> {
        let interner = DbInterner::new_with(db, Some(self.env.krate), self.env.block);
        let args = generic_args_from_tys(
            interner,
            alias.id.into(),
            std::iter::once(self.ty).chain(args.iter().map(|ty| ty.ty)),
        );
        let projection = Ty::new_alias(
            interner,
            AliasTyKind::Projection,
            AliasTy::new(interner, alias.id.into(), args),
        );
        let infcx = interner.infer_ctxt().build(TypingMode::PostAnalysis);
        let ty = structurally_normalize_ty(&infcx, projection, self.env.clone());
        if ty.is_ty_error() { None } else { Some(self.derived(ty)) }
    }
    pub fn is_copy(&self, db: &'db dyn HirDatabase) -> bool {
        let Some(copy_trait) = LangItem::Copy.resolve_trait(db, self.env.krate) else {
            return false;
        };
        self.impls_trait(db, copy_trait.into(), &[])
    }
    pub fn as_callable(&self, db: &'db dyn HirDatabase) -> Option<Callable<'db>> {
        let interner = DbInterner::new_with(db, None, None);
        let callee = match self.ty.kind() {
            TyKind::Closure(id, subst) => Callee::Closure(id.0, subst),
            TyKind::CoroutineClosure(id, subst) => Callee::CoroutineClosure(id.0, subst),
            TyKind::FnPtr(..) => Callee::FnPtr,
            TyKind::FnDef(id, _) => Callee::Def(id.0),
            TyKind::Ref(_, inner_ty, _) => return self.derived(inner_ty).as_callable(db),
            _ => {
                let (fn_trait, sig) = hir_ty::callable_sig_from_fn_trait(
                    self.ty,
                    self.env.clone(),
                    db,
                )?;
                return Some(Callable {
                    ty: self.clone(),
                    sig,
                    callee: Callee::FnImpl(fn_trait),
                    is_bound_method: false,
                });
            }
        };
        let sig = self.ty.callable_sig(interner)?;
        Some(Callable {
            ty: self.clone(),
            sig,
            callee,
            is_bound_method: false,
        })
    }
    pub fn is_closure(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Closure { .. })
    }
    pub fn as_closure(&self) -> Option<Closure<'db>> {
        match self.ty.kind() {
            TyKind::Closure(id, subst) => {
                Some(Closure {
                    id: AnyClosureId::ClosureId(id.0),
                    subst,
                })
            }
            TyKind::CoroutineClosure(id, subst) => {
                Some(Closure {
                    id: AnyClosureId::CoroutineClosureId(id.0),
                    subst,
                })
            }
            _ => None,
        }
    }
    pub fn is_fn(&self) -> bool {
        matches!(self.ty.kind(), TyKind::FnDef(..) | TyKind::FnPtr { .. })
    }
    pub fn is_array(&self) -> bool {
        matches!(self.ty.kind(), TyKind::Array(..))
    }
    pub fn is_packed(&self, db: &'db dyn HirDatabase) -> bool {
        let adt_id = match self.ty.kind() {
            TyKind::Adt(adt_def, ..) => adt_def.def_id().0,
            _ => return false,
        };
        let adt = adt_id.into();
        match adt {
            Adt::Struct(s) => s.repr(db).unwrap_or_default().pack.is_some(),
            _ => false,
        }
    }
    pub fn is_raw_ptr(&self) -> bool {
        matches!(self.ty.kind(), TyKind::RawPtr(..))
    }
    pub fn remove_raw_ptr(&self) -> Option<Type<'db>> {
        if let TyKind::RawPtr(ty, _) = self.ty.kind() {
            Some(self.derived(ty))
        } else {
            None
        }
    }
    pub fn contains_unknown(&self) -> bool {
        self.ty.references_non_lt_error()
    }
    pub fn fields(&self, db: &'db dyn HirDatabase) -> Vec<(Field, Self)> {
        let interner = DbInterner::new_with(db, None, None);
        let (variant_id, substs) = match self.ty.kind() {
            TyKind::Adt(adt_def, substs) => {
                let id = match adt_def.def_id().0 {
                    AdtId::StructId(id) => id.into(),
                    AdtId::UnionId(id) => id.into(),
                    AdtId::EnumId(_) => return Vec::new(),
                };
                (id, substs)
            }
            _ => return Vec::new(),
        };
        db.field_types(variant_id)
            .iter()
            .map(|(local_id, ty)| {
                let def = Field {
                    parent: variant_id.into(),
                    id: local_id,
                };
                let ty = ty.instantiate(interner, substs);
                (def, self.derived(ty))
            })
            .collect()
    }
    pub fn tuple_fields(&self, _db: &'db dyn HirDatabase) -> Vec<Self> {
        if let TyKind::Tuple(substs) = self.ty.kind() {
            substs.iter().map(|ty| self.derived(ty)).collect()
        } else {
            Vec::new()
        }
    }
    pub fn as_array(&self, db: &'db dyn HirDatabase) -> Option<(Self, usize)> {
        if let TyKind::Array(ty, len) = self.ty.kind() {
            try_const_usize(db, len).map(|it| (self.derived(ty), it as usize))
        } else {
            None
        }
    }
    pub fn fingerprint_for_trait_impl(&self) -> Option<SimplifiedType> {
        fast_reject::simplify_type(
            DbInterner::conjure(),
            self.ty,
            fast_reject::TreatParams::AsRigid,
        )
    }
    /// Returns types that this type dereferences to (including this type itself). The returned
    /// iterator won't yield the same type more than once even if the deref chain contains a cycle.
    pub fn autoderef(
        &self,
        db: &'db dyn HirDatabase,
    ) -> impl Iterator<Item = Type<'db>> + use<'_, 'db> {
        self.autoderef_(db).map(move |ty| self.derived(ty))
    }
    fn autoderef_(&self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Ty<'db>> {
        let interner = DbInterner::new_with(db, None, None);
        let canonical = hir_ty::replace_errors_with_variables(interner, &self.ty);
        autoderef(db, self.env.clone(), canonical)
    }
    pub fn iterate_assoc_items<T>(
        &self,
        db: &'db dyn HirDatabase,
        mut callback: impl FnMut(AssocItem) -> Option<T>,
    ) -> Option<T> {
        let mut slot = None;
        self.iterate_assoc_items_dyn(
            db,
            &mut |assoc_item_id| {
                slot = callback(assoc_item_id.into());
                slot.is_some()
            },
        );
        slot
    }
    fn iterate_assoc_items_dyn(
        &self,
        db: &'db dyn HirDatabase,
        callback: &mut dyn FnMut(AssocItemId) -> bool,
    ) {
        let mut handle_impls = |impls: &[ImplId]| {
            for &impl_def in impls {
                for &(_, item) in impl_def.impl_items(db).items.iter() {
                    if callback(item) {
                        return;
                    }
                }
            }
        };
        let interner = DbInterner::new_with(db, None, None);
        let Some(simplified_type) = fast_reject::simplify_type(
            interner,
            self.ty,
            fast_reject::TreatParams::AsRigid,
        ) else {
            return;
        };
        handle_impls(method_resolution::incoherent_inherent_impls(db, simplified_type));
        if let Some(module) = method_resolution::simplified_type_module(
            db,
            &simplified_type,
        ) {
            InherentImpls::for_each_crate_and_block(
                db,
                module.krate(),
                module.containing_block(),
                &mut |impls| {
                    handle_impls(impls.for_self_ty(&simplified_type));
                },
            );
        }
    }
    /// Iterates its type arguments
    ///
    /// It iterates the actual type arguments when concrete types are used
    /// and otherwise the generic names.
    /// It does not include `const` arguments.
    ///
    /// For code, such as:
    /// ```text
    /// struct Foo<T, U>
    ///
    /// impl<U> Foo<String, U>
    /// ```
    ///
    /// It iterates:
    /// ```text
    /// - "String"
    /// - "U"
    /// ```
    pub fn type_arguments(&self) -> impl Iterator<Item = Type<'db>> + '_ {
        match self.ty.strip_references().kind() {
            TyKind::Adt(_, substs) => {
                Either::Left(substs.types().map(move |ty| self.derived(ty)))
            }
            TyKind::Tuple(substs) => {
                Either::Right(
                    Either::Left(substs.iter().map(move |ty| self.derived(ty))),
                )
            }
            _ => Either::Right(Either::Right(std::iter::empty())),
        }
    }
    /// Iterates its type and const arguments
    ///
    /// It iterates the actual type and const arguments when concrete types
    /// are used and otherwise the generic names.
    ///
    /// For code, such as:
    /// ```text
    /// struct Foo<T, const U: usize, const X: usize>
    ///
    /// impl<U> Foo<String, U, 12>
    /// ```
    ///
    /// It iterates:
    /// ```text
    /// - "String"
    /// - "U"
    /// - "12"
    /// ```
    pub fn type_and_const_arguments<'a>(
        &'a self,
        db: &'a dyn HirDatabase,
        display_target: DisplayTarget,
    ) -> impl Iterator<Item = SmolStr> + 'a {
        self.ty
            .strip_references()
            .as_adt()
            .into_iter()
            .flat_map(|(_, substs)| substs.iter())
            .filter_map(move |arg| match arg {
                GenericArg::Ty(ty) => {
                    Some(format_smolstr!("{}", ty.display(db, display_target)))
                }
                GenericArg::Const(const_) => {
                    Some(format_smolstr!("{}", const_.display(db, display_target)))
                }
                GenericArg::Lifetime(_) => None,
            })
    }
    /// Combines lifetime indicators, type and constant parameters into a single `Iterator`
    pub fn generic_parameters<'a>(
        &'a self,
        db: &'a dyn HirDatabase,
        display_target: DisplayTarget,
    ) -> impl Iterator<Item = SmolStr> + 'a {
        self.as_adt()
            .and_then(|a| {
                a.lifetime(db)
                    .map(|lt| lt.name.display_no_db(Edition::Edition2015).to_smolstr())
            })
            .into_iter()
            .chain(self.type_and_const_arguments(db, display_target))
    }
    pub fn iterate_method_candidates_with_traits<T>(
        &self,
        db: &'db dyn HirDatabase,
        scope: &SemanticsScope<'_>,
        traits_in_scope: &FxHashSet<TraitId>,
        name: Option<&Name>,
        mut callback: impl FnMut(Function) -> Option<T>,
    ) -> Option<T> {
        let _p = tracing::info_span!("iterate_method_candidates_with_traits").entered();
        let mut slot = None;
        self.iterate_method_candidates_split_inherent(
            db,
            scope,
            traits_in_scope,
            name,
            |f| {
                match callback(f) {
                    it @ Some(_) => {
                        slot = it;
                        ControlFlow::Break(())
                    }
                    None => ControlFlow::Continue(()),
                }
            },
        );
        slot
    }
    pub fn iterate_method_candidates<T>(
        &self,
        db: &'db dyn HirDatabase,
        scope: &SemanticsScope<'_>,
        name: Option<&Name>,
        callback: impl FnMut(Function) -> Option<T>,
    ) -> Option<T> {
        self.iterate_method_candidates_with_traits(
            db,
            scope,
            &scope.visible_traits().0,
            name,
            callback,
        )
    }
    fn with_method_resolution<R>(
        &self,
        db: &'db dyn HirDatabase,
        resolver: &Resolver<'db>,
        traits_in_scope: &FxHashSet<TraitId>,
        f: impl FnOnce(&MethodResolutionContext<'_, 'db>) -> R,
    ) -> R {
        let module = resolver.module();
        let interner = DbInterner::new_with(
            db,
            Some(module.krate()),
            module.containing_block(),
        );
        let infcx = interner.infer_ctxt().build(TypingMode::PostAnalysis);
        let unstable_features = MethodResolutionUnstableFeatures::from_def_map(
            resolver.top_level_def_map(),
        );
        let environment = resolver
            .generic_def()
            .map_or_else(
                || TraitEnvironment::empty(module.krate()),
                |d| db.trait_environment(d),
            );
        let ctx = MethodResolutionContext {
            infcx: &infcx,
            resolver,
            env: &environment,
            traits_in_scope,
            edition: resolver.krate().data(db).edition,
            unstable_features: &unstable_features,
        };
        f(&ctx)
    }
    /// Allows you to treat inherent and non-inherent methods differently.
    ///
    /// Note that inherent methods may actually be trait methods! For example, in `dyn Trait`, the trait's methods
    /// are considered inherent methods.
    pub fn iterate_method_candidates_split_inherent(
        &self,
        db: &'db dyn HirDatabase,
        scope: &SemanticsScope<'_>,
        traits_in_scope: &FxHashSet<TraitId>,
        name: Option<&Name>,
        mut callback: impl MethodCandidateCallback,
    ) {
        let _p = tracing::info_span!(
            "iterate_method_candidates_split_inherent", traits_in_scope = traits_in_scope
            .len(), ? name,
        )
            .entered();
        self.with_method_resolution(
            db,
            scope.resolver(),
            traits_in_scope,
            |ctx| {
                let canonical = hir_ty::replace_errors_with_variables(
                    ctx.infcx.interner,
                    &self.ty,
                );
                let (self_ty, _) = ctx.infcx.instantiate_canonical(&canonical);
                match name {
                    Some(name) => {
                        match ctx
                            .probe_for_name(
                                method_resolution::Mode::MethodCall,
                                name.clone(),
                                self_ty,
                            )
                        {
                            Ok(candidate)
                            | Err(
                                method_resolution::MethodError::PrivateMatch(candidate),
                            ) => {
                                let method_resolution::CandidateId::FunctionId(id) = candidate
                                    .item else {
                                    unreachable!(
                                        "`Mode::MethodCall` can only return functions"
                                    );
                                };
                                let id = Function { id };
                                match candidate.kind {
                                    method_resolution::PickKind::InherentImplPick(_)
                                    | method_resolution::PickKind::ObjectPick(..)
                                    | method_resolution::PickKind::WhereClausePick(..) => {
                                        _ = callback.on_inherent_method(id);
                                    }
                                    method_resolution::PickKind::TraitPick(..) => {
                                        _ = callback.on_trait_method(id);
                                    }
                                }
                            }
                            Err(_) => {}
                        };
                    }
                    None => {
                        _ = ctx
                            .probe_all(method_resolution::Mode::MethodCall, self_ty)
                            .try_for_each(|candidate| {
                                let method_resolution::CandidateId::FunctionId(id) = candidate
                                    .candidate
                                    .item else {
                                    unreachable!(
                                        "`Mode::MethodCall` can only return functions"
                                    );
                                };
                                let id = Function { id };
                                match candidate.candidate.kind {
                                    method_resolution::CandidateKind::InherentImplCandidate {
                                        ..
                                    }
                                    | method_resolution::CandidateKind::ObjectCandidate(..)
                                    | method_resolution::CandidateKind::WhereClauseCandidate(
                                        ..,
                                    ) => callback.on_inherent_method(id),
                                    method_resolution::CandidateKind::TraitCandidate(..) => {
                                        callback.on_trait_method(id)
                                    }
                                }
                            });
                    }
                }
            },
        )
    }
    #[tracing::instrument(skip_all, fields(name = ?name))]
    pub fn iterate_path_candidates<T>(
        &self,
        db: &'db dyn HirDatabase,
        scope: &SemanticsScope<'_>,
        traits_in_scope: &FxHashSet<TraitId>,
        name: Option<&Name>,
        mut callback: impl FnMut(AssocItem) -> Option<T>,
    ) -> Option<T> {
        let _p = tracing::info_span!("iterate_path_candidates").entered();
        let mut slot = None;
        self.iterate_path_candidates_split_inherent(
            db,
            scope,
            traits_in_scope,
            name,
            |item| {
                match callback(item) {
                    it @ Some(_) => {
                        slot = it;
                        ControlFlow::Break(())
                    }
                    None => ControlFlow::Continue(()),
                }
            },
        );
        slot
    }
    /// Iterates over inherent methods.
    ///
    /// In some circumstances, inherent methods methods may actually be trait methods!
    /// For example, when `dyn Trait` is a receiver, _trait_'s methods would be considered
    /// to be inherent methods.
    #[tracing::instrument(skip_all, fields(name = ?name))]
    pub fn iterate_path_candidates_split_inherent(
        &self,
        db: &'db dyn HirDatabase,
        scope: &SemanticsScope<'_>,
        traits_in_scope: &FxHashSet<TraitId>,
        name: Option<&Name>,
        mut callback: impl PathCandidateCallback,
    ) {
        let _p = tracing::info_span!(
            "iterate_path_candidates_split_inherent", traits_in_scope = traits_in_scope
            .len(), ? name,
        )
            .entered();
        self.with_method_resolution(
            db,
            scope.resolver(),
            traits_in_scope,
            |ctx| {
                let canonical = hir_ty::replace_errors_with_variables(
                    ctx.infcx.interner,
                    &self.ty,
                );
                let (self_ty, _) = ctx.infcx.instantiate_canonical(&canonical);
                match name {
                    Some(name) => {
                        match ctx
                            .probe_for_name(
                                method_resolution::Mode::MethodCall,
                                name.clone(),
                                self_ty,
                            )
                        {
                            Ok(candidate)
                            | Err(
                                method_resolution::MethodError::PrivateMatch(candidate),
                            ) => {
                                let id = candidate.item.into();
                                match candidate.kind {
                                    method_resolution::PickKind::InherentImplPick(_)
                                    | method_resolution::PickKind::ObjectPick(..)
                                    | method_resolution::PickKind::WhereClausePick(..) => {
                                        _ = callback.on_inherent_item(id);
                                    }
                                    method_resolution::PickKind::TraitPick(..) => {
                                        _ = callback.on_trait_item(id);
                                    }
                                }
                            }
                            Err(_) => {}
                        };
                    }
                    None => {
                        _ = ctx
                            .probe_all(method_resolution::Mode::Path, self_ty)
                            .try_for_each(|candidate| {
                                let id = candidate.candidate.item.into();
                                match candidate.candidate.kind {
                                    method_resolution::CandidateKind::InherentImplCandidate {
                                        ..
                                    }
                                    | method_resolution::CandidateKind::ObjectCandidate(..)
                                    | method_resolution::CandidateKind::WhereClauseCandidate(
                                        ..,
                                    ) => callback.on_inherent_item(id),
                                    method_resolution::CandidateKind::TraitCandidate(..) => {
                                        callback.on_trait_item(id)
                                    }
                                }
                            });
                    }
                }
            },
        )
    }
    pub fn as_adt(&self) -> Option<Adt> {
        let (adt, _subst) = self.ty.as_adt()?;
        Some(adt.into())
    }
    pub fn as_builtin(&self) -> Option<BuiltinType> {
        self.ty.as_builtin().map(|inner| BuiltinType { inner })
    }
    pub fn as_dyn_trait(&self) -> Option<Trait> {
        self.ty.dyn_trait().map(Into::into)
    }
    /// If a type can be represented as `dyn Trait`, returns all traits accessible via this type,
    /// or an empty iterator otherwise.
    pub fn applicable_inherent_traits(
        &self,
        db: &'db dyn HirDatabase,
    ) -> impl Iterator<Item = Trait> {
        let _p = tracing::info_span!("applicable_inherent_traits").entered();
        self.autoderef_(db)
            .filter_map(|ty| ty.dyn_trait())
            .flat_map(move |dyn_trait_id| hir_ty::all_super_traits(db, dyn_trait_id))
            .map(Trait::from)
    }
    pub fn env_traits(&self, db: &'db dyn HirDatabase) -> impl Iterator<Item = Trait> {
        let _p = tracing::info_span!("env_traits").entered();
        self.autoderef_(db)
            .filter(|ty| matches!(ty.kind(), TyKind::Param(_)))
            .flat_map(|ty| {
                self.env
                    .traits_in_scope_from_clauses(ty)
                    .flat_map(|t| hir_ty::all_super_traits(db, t))
            })
            .map(Trait::from)
    }
    pub fn as_impl_traits(
        &self,
        db: &'db dyn HirDatabase,
    ) -> Option<impl Iterator<Item = Trait>> {
        self.ty
            .impl_trait_bounds(db)
            .map(|it| {
                it.into_iter()
                    .filter_map(|pred| match pred.kind().skip_binder() {
                        ClauseKind::Trait(trait_ref) => {
                            Some(Trait::from(trait_ref.def_id().0))
                        }
                        _ => None,
                    })
            })
    }
    pub fn as_associated_type_parent_trait(
        &self,
        db: &'db dyn HirDatabase,
    ) -> Option<Trait> {
        let TyKind::Alias(AliasTyKind::Projection, alias) = self.ty.kind() else {
            return None
        };
        match alias.def_id.expect_type_alias().loc(db).container {
            ItemContainerId::TraitId(id) => Some(Trait { id }),
            _ => None,
        }
    }
    fn derived(&self, ty: Ty<'db>) -> Self {
        Type { env: self.env.clone(), ty }
    }
    /// Visits every type, including generic arguments, in this type. `callback` is called with type
    /// itself first, and then with its generic arguments.
    pub fn walk(&self, db: &'db dyn HirDatabase, callback: impl FnMut(Type<'db>)) {
        struct Visitor<'db, F> {
            db: &'db dyn HirDatabase,
            env: Arc<TraitEnvironment<'db>>,
            callback: F,
            visited: FxHashSet<Ty<'db>>,
        }
        impl<'db, F> TypeVisitor<DbInterner<'db>> for Visitor<'db, F>
        where
            F: FnMut(Type<'db>),
        {
            type Result = ();
            fn visit_ty(&mut self, ty: Ty<'db>) -> Self::Result {
                if !self.visited.insert(ty) {
                    return;
                }
                (self.callback)(Type { env: self.env.clone(), ty });
                if let Some(bounds) = ty.impl_trait_bounds(self.db) {
                    bounds.visit_with(self);
                }
                ty.super_visit_with(self);
            }
        }
        let mut visitor = Visitor {
            db,
            env: self.env.clone(),
            callback,
            visited: FxHashSet::default(),
        };
        self.ty.visit_with(&mut visitor);
    }
    /// Check if type unifies with another type.
    ///
    /// Note that we consider placeholder types to unify with everything.
    /// For example `Option<T>` and `Option<U>` unify although there is unresolved goal `T = U`.
    pub fn could_unify_with(&self, db: &'db dyn HirDatabase, other: &Type<'db>) -> bool {
        let interner = DbInterner::new_with(db, None, None);
        let tys = hir_ty::replace_errors_with_variables(interner, &(self.ty, other.ty));
        hir_ty::could_unify(db, self.env.clone(), &tys)
    }
    /// Check if type unifies with another type eagerly making sure there are no unresolved goals.
    ///
    /// This means that placeholder types are not considered to unify if there are any bounds set on
    /// them. For example `Option<T>` and `Option<U>` do not unify as we cannot show that `T = U`
    pub fn could_unify_with_deeply(
        &self,
        db: &'db dyn HirDatabase,
        other: &Type<'db>,
    ) -> bool {
        let interner = DbInterner::new_with(db, None, None);
        let tys = hir_ty::replace_errors_with_variables(interner, &(self.ty, other.ty));
        hir_ty::could_unify_deeply(db, self.env.clone(), &tys)
    }
    pub fn could_coerce_to(&self, db: &'db dyn HirDatabase, to: &Type<'db>) -> bool {
        let interner = DbInterner::new_with(db, None, None);
        let tys = hir_ty::replace_errors_with_variables(interner, &(self.ty, to.ty));
        hir_ty::could_coerce(db, self.env.clone(), &tys)
    }
    pub fn as_type_param(&self, _db: &'db dyn HirDatabase) -> Option<TypeParam> {
        match self.ty.kind() {
            TyKind::Param(param) => Some(TypeParam { id: param.id }),
            _ => None,
        }
    }
    /// Returns unique `GenericParam`s contained in this type.
    pub fn generic_params(&self, db: &'db dyn HirDatabase) -> FxHashSet<GenericParam> {
        hir_ty::collect_params(&self.ty)
            .into_iter()
            .map(|id| TypeOrConstParam { id }.split(db).either_into())
            .collect()
    }
    pub fn layout(&self, db: &'db dyn HirDatabase) -> Result<Layout, LayoutError> {
        db.layout_of_ty(self.ty, self.env.clone())
            .map(|layout| Layout(layout, db.target_data_layout(self.env.krate).unwrap()))
    }
    pub fn drop_glue(&self, db: &'db dyn HirDatabase) -> DropGlue {
        let interner = DbInterner::new_with(db, Some(self.env.krate), self.env.block);
        let infcx = interner.infer_ctxt().build(TypingMode::PostAnalysis);
        hir_ty::drop::has_drop_glue(&infcx, self.ty, self.env.clone())
    }
}
