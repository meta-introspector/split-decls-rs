macro_rules! deps {
    () => {
        HirDatabase!();
        LifetimeElisionKind!();
        TyLoweringContext!();
        Diagnostics!();
        EarlyBinder!();
        ImplTraitLoweringMode!();
    };
}

macro_rules! type_for_type_alias_with_diagnostics_query {
    () => {
        deps!();
        pub (crate) fn type_for_type_alias_with_diagnostics_query < 'db > (db : & 'db dyn HirDatabase , t : TypeAliasId ,) -> (EarlyBinder < 'db , Ty < 'db > > , Diagnostics) { let type_alias_data = db . type_alias_signature (t) ; let mut diags = None ; let resolver = t . resolver (db) ; let interner = DbInterner :: new_with (db , Some (resolver . krate ()) , None) ; let inner = if type_alias_data . flags . contains (TypeAliasFlags :: IS_EXTERN) { EarlyBinder :: bind (Ty :: new_foreign (interner , t . into ())) } else { let mut ctx = TyLoweringContext :: new (db , & resolver , & type_alias_data . store , t . into () , LifetimeElisionKind :: AnonymousReportError ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) ; let res = EarlyBinder :: bind (type_alias_data . ty . map (| type_ref | ctx . lower_ty (type_ref)) . unwrap_or_else (| | Ty :: new_error (interner , ErrorGuaranteed)) ,) ; diags = create_diagnostics (ctx . diagnostics) ; res } ; (inner , diags) }
    };
}

type_for_type_alias_with_diagnostics_query!();