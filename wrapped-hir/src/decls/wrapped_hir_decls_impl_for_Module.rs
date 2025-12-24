use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Module {
    /// Name of this module.
    pub fn name(self, db: &dyn HirDatabase) -> Option<Name> {
        self.id.name(db)
    }
    /// Returns the crate this module is part of.
    pub fn krate(self) -> Crate {
        Crate { id: self.id.krate() }
    }
    /// Topmost parent of this module. Every module has a `crate_root`, but some
    /// might be missing `krate`. This can happen if a module's file is not included
    /// in the module tree of any target in `Cargo.toml`.
    pub fn crate_root(self, db: &dyn HirDatabase) -> Module {
        let def_map = crate_def_map(db, self.id.krate());
        Module {
            id: def_map.crate_root().into(),
        }
    }
    pub fn is_crate_root(self) -> bool {
        DefMap::ROOT == self.id.local_id
    }
    /// Iterates over all child modules.
    pub fn children(self, db: &dyn HirDatabase) -> impl Iterator<Item = Module> {
        let def_map = self.id.def_map(db);
        let children = def_map[self.id.local_id]
            .children
            .values()
            .map(|module_id| Module {
                id: def_map.module_id(*module_id),
            })
            .collect::<Vec<_>>();
        children.into_iter()
    }
    /// Finds a parent module.
    pub fn parent(self, db: &dyn HirDatabase) -> Option<Module> {
        let def_map = self.id.def_map(db);
        let parent_id = def_map.containing_module(self.id.local_id)?;
        Some(Module { id: parent_id })
    }
    /// Finds nearest non-block ancestor `Module` (`self` included).
    pub fn nearest_non_block_module(self, db: &dyn HirDatabase) -> Module {
        let mut id = self.id;
        while id.is_block_module() {
            id = id.containing_module(db).expect("block without parent module");
        }
        Module { id }
    }
    pub fn path_to_root(self, db: &dyn HirDatabase) -> Vec<Module> {
        let mut res = vec![self];
        let mut curr = self;
        while let Some(next) = curr.parent(db) {
            res.push(next);
            curr = next;
        }
        res
    }
    /// Returns a `ModuleScope`: a set of items, visible in this module.
    pub fn scope(
        self,
        db: &dyn HirDatabase,
        visible_from: Option<Module>,
    ) -> Vec<(Name, ScopeDef)> {
        self.id
            .def_map(db)[self.id.local_id]
            .scope
            .entries()
            .filter_map(|(name, def)| {
                if let Some(m) = visible_from {
                    let filtered = def
                        .filter_visibility(|vis| vis.is_visible_from(db, m.id));
                    if filtered.is_none() && !def.is_none() {
                        None
                    } else {
                        Some((name, filtered))
                    }
                } else {
                    Some((name, def))
                }
            })
            .flat_map(|(name, def)| {
                ScopeDef::all_items(def)
                    .into_iter()
                    .map(move |item| (name.clone(), item))
            })
            .collect()
    }
    pub fn resolve_mod_path(
        &self,
        db: &dyn HirDatabase,
        segments: impl IntoIterator<Item = Name>,
    ) -> Option<impl Iterator<Item = ItemInNs>> {
        let items = self
            .id
            .resolver(db)
            .resolve_module_path_in_items(
                db,
                &ModPath::from_segments(PathKind::Plain, segments),
            );
        Some(items.iter_items().map(|(item, _)| item.into()))
    }
    /// Fills `acc` with the module's diagnostics.
    pub fn diagnostics<'db>(
        self,
        db: &'db dyn HirDatabase,
        acc: &mut Vec<AnyDiagnostic<'db>>,
        style_lints: bool,
    ) {
        let _p = tracing::info_span!("diagnostics", name = ? self.name(db)).entered();
        let edition = self.id.krate().data(db).edition;
        let def_map = self.id.def_map(db);
        for diag in def_map.diagnostics() {
            if diag.in_module != self.id.local_id {
                continue;
            }
            emit_def_diagnostic(db, acc, diag, edition);
        }
        if !self.id.is_block_module() {
            let scope = &def_map[self.id.local_id].scope;
            scope.all_macro_calls().for_each(|it| macro_call_diagnostics(db, it, acc));
        }
        for def in self.declarations(db) {
            match def {
                ModuleDef::Module(m) => {
                    if def_map[m.id.local_id].origin.is_inline() {
                        m.diagnostics(db, acc, style_lints)
                    }
                    acc.extend(def.diagnostics(db, style_lints))
                }
                ModuleDef::Trait(t) => {
                    for diag in TraitItems::query_with_diagnostics(db, t.id).1.iter() {
                        emit_def_diagnostic(db, acc, diag, edition);
                    }
                    for item in t.items(db) {
                        item.diagnostics(db, acc, style_lints);
                    }
                    t.all_macro_calls(db)
                        .iter()
                        .for_each(|&(_ast, call_id)| macro_call_diagnostics(
                            db,
                            call_id,
                            acc,
                        ));
                    acc.extend(def.diagnostics(db, style_lints))
                }
                ModuleDef::Adt(adt) => {
                    match adt {
                        Adt::Struct(s) => {
                            let source_map = db.struct_signature_with_source_map(s.id).1;
                            expr_store_diagnostics(db, acc, &source_map);
                            let source_map = &s.id.fields_with_source_map(db).1;
                            expr_store_diagnostics(db, acc, source_map);
                            push_ty_diagnostics(
                                db,
                                acc,
                                db.field_types_with_diagnostics(s.id.into()).1,
                                source_map,
                            );
                        }
                        Adt::Union(u) => {
                            let source_map = db.union_signature_with_source_map(u.id).1;
                            expr_store_diagnostics(db, acc, &source_map);
                            let source_map = &u.id.fields_with_source_map(db).1;
                            expr_store_diagnostics(db, acc, source_map);
                            push_ty_diagnostics(
                                db,
                                acc,
                                db.field_types_with_diagnostics(u.id.into()).1,
                                source_map,
                            );
                        }
                        Adt::Enum(e) => {
                            let source_map = db.enum_signature_with_source_map(e.id).1;
                            expr_store_diagnostics(db, acc, &source_map);
                            let (variants, diagnostics) = e
                                .id
                                .enum_variants_with_diagnostics(db);
                            let file = e.id.lookup(db).id.file_id;
                            let ast_id_map = db.ast_id_map(file);
                            if let Some(diagnostics) = &diagnostics {
                                for diag in diagnostics.iter() {
                                    acc.push(
                                        InactiveCode {
                                            node: InFile::new(
                                                file,
                                                ast_id_map.get(diag.ast_id).syntax_node_ptr(),
                                            ),
                                            cfg: diag.cfg.clone(),
                                            opts: diag.opts.clone(),
                                        }
                                            .into(),
                                    );
                                }
                            }
                            for &(v, _, _) in &variants.variants {
                                let source_map = &v.fields_with_source_map(db).1;
                                push_ty_diagnostics(
                                    db,
                                    acc,
                                    db.field_types_with_diagnostics(v.into()).1,
                                    source_map,
                                );
                                expr_store_diagnostics(db, acc, source_map);
                            }
                        }
                    }
                    acc.extend(def.diagnostics(db, style_lints))
                }
                ModuleDef::Macro(m) => emit_macro_def_diagnostics(db, acc, m),
                ModuleDef::TypeAlias(type_alias) => {
                    let source_map = db
                        .type_alias_signature_with_source_map(type_alias.id)
                        .1;
                    expr_store_diagnostics(db, acc, &source_map);
                    push_ty_diagnostics(
                        db,
                        acc,
                        db.type_for_type_alias_with_diagnostics(type_alias.id).1,
                        &source_map,
                    );
                    acc.extend(def.diagnostics(db, style_lints));
                }
                _ => acc.extend(def.diagnostics(db, style_lints)),
            }
        }
        self.legacy_macros(db)
            .into_iter()
            .for_each(|m| emit_macro_def_diagnostics(db, acc, m));
        let interner = DbInterner::new_with(
            db,
            Some(self.id.krate()),
            self.id.containing_block(),
        );
        let infcx = interner.infer_ctxt().build(TypingMode::non_body_analysis());
        let mut impl_assoc_items_scratch = vec![];
        for impl_def in self.impl_defs(db) {
            GenericDef::Impl(impl_def).diagnostics(db, acc);
            let loc = impl_def.id.lookup(db);
            let (impl_signature, source_map) = db
                .impl_signature_with_source_map(impl_def.id);
            expr_store_diagnostics(db, acc, &source_map);
            let file_id = loc.id.file_id;
            if file_id
                .macro_file()
                .is_some_and(|it| it.kind(db) == MacroKind::DeriveBuiltIn)
            {
                continue;
            }
            impl_def
                .all_macro_calls(db)
                .iter()
                .for_each(|&(_ast, call_id)| macro_call_diagnostics(db, call_id, acc));
            let ast_id_map = db.ast_id_map(file_id);
            for diag in impl_def.id.impl_items_with_diagnostics(db).1.iter() {
                emit_def_diagnostic(db, acc, diag, edition);
            }
            if impl_signature.target_trait.is_none()
                && !is_inherent_impl_coherent(db, def_map, impl_def.id)
            {
                acc.push(
                    IncoherentImpl {
                        impl_: ast_id_map.get(loc.id.value),
                        file_id,
                    }
                        .into(),
                )
            }
            if !impl_def.check_orphan_rules(db) {
                acc.push(
                    TraitImplOrphan {
                        impl_: ast_id_map.get(loc.id.value),
                        file_id,
                    }
                        .into(),
                )
            }
            let trait_ = impl_def.trait_(db);
            let mut trait_is_unsafe = trait_.is_some_and(|t| t.is_unsafe(db));
            let impl_is_negative = impl_def.is_negative(db);
            let impl_is_unsafe = impl_def.is_unsafe(db);
            let trait_is_unresolved = trait_.is_none()
                && impl_signature.target_trait.is_some();
            if trait_is_unresolved {
                trait_is_unsafe = impl_is_unsafe;
            }
            let drop_maybe_dangle = (|| {
                let trait_ = trait_?;
                let drop_trait = LangItem::Drop.resolve_trait(db, self.krate().into())?;
                if drop_trait != trait_.into() {
                    return None;
                }
                let parent = impl_def.id.into();
                let generic_params = db.generic_params(parent);
                let lifetime_params = generic_params
                    .iter_lt()
                    .map(|(local_id, _)| {
                        GenericParamId::LifetimeParamId(LifetimeParamId {
                            parent,
                            local_id,
                        })
                    });
                let type_params = generic_params
                    .iter_type_or_consts()
                    .filter(|(_, it)| it.type_param().is_some())
                    .map(|(local_id, _)| {
                        GenericParamId::TypeParamId(
                            TypeParamId::from_unchecked(TypeOrConstParamId {
                                parent,
                                local_id,
                            }),
                        )
                    });
                let res = type_params
                    .chain(lifetime_params)
                    .any(|p| {
                        db.attrs(AttrDefId::GenericParamId(p))
                            .by_key(sym::may_dangle)
                            .exists()
                    });
                Some(res)
            })()
                .unwrap_or(false);
            match (
                impl_is_unsafe,
                trait_is_unsafe,
                impl_is_negative,
                drop_maybe_dangle,
            ) {
                (true, _, true, _) | (true, false, _, false) => {
                    acc.push(
                        TraitImplIncorrectSafety {
                            impl_: ast_id_map.get(loc.id.value),
                            file_id,
                            should_be_safe: true,
                        }
                            .into(),
                    )
                }
                (false, true, false, _) | (false, false, _, true) => {
                    acc.push(
                        TraitImplIncorrectSafety {
                            impl_: ast_id_map.get(loc.id.value),
                            file_id,
                            should_be_safe: false,
                        }
                            .into(),
                    )
                }
                _ => {}
            };
            if let (false, Some(trait_)) = (impl_is_negative, trait_) {
                let items = &trait_.id.trait_items(db).items;
                let required_items = items
                    .iter()
                    .filter(|&(_, assoc)| match *assoc {
                        AssocItemId::FunctionId(it) => {
                            !db.function_signature(it).has_body()
                        }
                        AssocItemId::ConstId(id) => !db.const_signature(id).has_body(),
                        AssocItemId::TypeAliasId(it) => {
                            db.type_alias_signature(it).ty.is_none()
                        }
                    });
                impl_assoc_items_scratch
                    .extend(impl_def.id.impl_items(db).items.iter().cloned());
                let redundant = impl_assoc_items_scratch
                    .iter()
                    .filter(|(name, id)| {
                        !items
                            .iter()
                            .any(|(impl_name, impl_item)| {
                                discriminant(impl_item) == discriminant(id)
                                    && impl_name == name
                            })
                    })
                    .map(|(name, item)| (name.clone(), AssocItem::from(*item)));
                for (name, assoc_item) in redundant {
                    acc.push(
                        TraitImplRedundantAssocItems {
                            trait_,
                            file_id,
                            impl_: ast_id_map.get(loc.id.value),
                            assoc_item: (name, assoc_item),
                        }
                            .into(),
                    )
                }
                let mut missing: Vec<_> = required_items
                    .filter(|(name, id)| {
                        !impl_assoc_items_scratch
                            .iter()
                            .any(|(impl_name, impl_item)| {
                                discriminant(impl_item) == discriminant(id)
                                    && impl_name == name
                            })
                    })
                    .map(|(name, item)| (name.clone(), AssocItem::from(*item)))
                    .collect();
                if !missing.is_empty() {
                    let self_ty = db.impl_self_ty(impl_def.id).instantiate_identity();
                    let self_ty = structurally_normalize_ty(
                        &infcx,
                        self_ty,
                        db.trait_environment(impl_def.id.into()),
                    );
                    let self_ty_is_guaranteed_unsized = matches!(
                        self_ty.kind(), TyKind::Dynamic(..) | TyKind::Slice(..) |
                        TyKind::Str
                    );
                    if self_ty_is_guaranteed_unsized {
                        missing
                            .retain(|(_, assoc_item)| {
                                let assoc_item = match *assoc_item {
                                    AssocItem::Function(it) => it.id.into(),
                                    AssocItem::Const(it) => it.id.into(),
                                    AssocItem::TypeAlias(it) => it.id.into(),
                                };
                                !hir_ty::dyn_compatibility::generics_require_sized_self(
                                    db,
                                    assoc_item,
                                )
                            });
                    }
                }
                if !missing.is_empty() {
                    acc.push(
                        TraitImplMissingAssocItems {
                            impl_: ast_id_map.get(loc.id.value),
                            file_id,
                            missing,
                        }
                            .into(),
                    )
                }
                impl_assoc_items_scratch.clear();
            }
            push_ty_diagnostics(
                db,
                acc,
                db.impl_self_ty_with_diagnostics(impl_def.id).1,
                &source_map,
            );
            push_ty_diagnostics(
                db,
                acc,
                db.impl_trait_with_diagnostics(impl_def.id).and_then(|it| it.1),
                &source_map,
            );
            for &(_, item) in impl_def.id.impl_items(db).items.iter() {
                AssocItem::from(item).diagnostics(db, acc, style_lints);
            }
        }
    }
    pub fn declarations(self, db: &dyn HirDatabase) -> Vec<ModuleDef> {
        let def_map = self.id.def_map(db);
        let scope = &def_map[self.id.local_id].scope;
        scope
            .declarations()
            .map(ModuleDef::from)
            .chain(scope.unnamed_consts().map(|id| ModuleDef::Const(Const::from(id))))
            .collect()
    }
    pub fn legacy_macros(self, db: &dyn HirDatabase) -> Vec<Macro> {
        let def_map = self.id.def_map(db);
        let scope = &def_map[self.id.local_id].scope;
        scope.legacy_macros().flat_map(|(_, it)| it).map(|&it| it.into()).collect()
    }
    pub fn impl_defs(self, db: &dyn HirDatabase) -> Vec<Impl> {
        let def_map = self.id.def_map(db);
        def_map[self.id.local_id].scope.impls().map(Impl::from).collect()
    }
    /// Finds a path that can be used to refer to the given item from within
    /// this module, if possible.
    pub fn find_path(
        self,
        db: &dyn DefDatabase,
        item: impl Into<ItemInNs>,
        cfg: FindPathConfig,
    ) -> Option<ModPath> {
        hir_def::find_path::find_path(
            db,
            item.into().into(),
            self.into(),
            PrefixKind::Plain,
            false,
            cfg,
        )
    }
    /// Finds a path that can be used to refer to the given item from within
    /// this module, if possible. This is used for returning import paths for use-statements.
    pub fn find_use_path(
        self,
        db: &dyn DefDatabase,
        item: impl Into<ItemInNs>,
        prefix_kind: PrefixKind,
        cfg: FindPathConfig,
    ) -> Option<ModPath> {
        hir_def::find_path::find_path(
            db,
            item.into().into(),
            self.into(),
            prefix_kind,
            true,
            cfg,
        )
    }
}
