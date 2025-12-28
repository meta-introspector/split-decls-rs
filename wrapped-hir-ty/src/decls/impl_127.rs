macro_rules! deps {
    () => {
        HirDatabase!();
        ImplTraits!();
        TyLoweringContext!();
        EarlyBinder!();
        LifetimeElisionKind!();
        ImplTraitLoweringMode!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        # [salsa :: tracked] impl < 'db > ImplTraits < 'db > { # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub (crate) fn return_type_impl_traits (db : & 'db dyn HirDatabase , def : hir_def :: FunctionId ,) -> Option < Box < EarlyBinder < 'db , ImplTraits < 'db > > > > { let data = db . function_signature (def) ; let resolver = def . resolver (db) ; let mut ctx_ret = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: Infer ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) ; if let Some (ret_type) = data . ret_type { let _ret = ctx_ret . lower_ty (ret_type) ; } let mut return_type_impl_traits = ImplTraits { impl_traits : ctx_ret . impl_trait_mode . opaque_type_data } ; if return_type_impl_traits . impl_traits . is_empty () { None } else { return_type_impl_traits . impl_traits . shrink_to_fit () ; Some (Box :: new (EarlyBinder :: bind (return_type_impl_traits))) } } # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub (crate) fn type_alias_impl_traits (db : & 'db dyn HirDatabase , def : hir_def :: TypeAliasId ,) -> Option < Box < EarlyBinder < 'db , ImplTraits < 'db > > > > { let data = db . type_alias_signature (def) ; let resolver = def . resolver (db) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: AnonymousReportError ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) ; if let Some (type_ref) = data . ty { let _ty = ctx . lower_ty (type_ref) ; } let mut type_alias_impl_traits = ImplTraits { impl_traits : ctx . impl_trait_mode . opaque_type_data } ; if type_alias_impl_traits . impl_traits . is_empty () { None } else { type_alias_impl_traits . impl_traits . shrink_to_fit () ; Some (Box :: new (EarlyBinder :: bind (type_alias_impl_traits))) } } }
    };
}

impl_127!();