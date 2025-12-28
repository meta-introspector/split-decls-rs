macro_rules! deps {
    () => {
        Diagnostics!();
        TyLoweringContext!();
        EarlyBinder!();
        HirDatabase!();
        LifetimeElisionKind!();
    };
}

macro_rules! type_alias_bounds_with_diagnostics {
    () => {
        deps!();
        # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub fn type_alias_bounds_with_diagnostics < 'db > (db : & 'db dyn HirDatabase , type_alias : TypeAliasId ,) -> (EarlyBinder < 'db , Box < [Clause < 'db >] > > , Diagnostics) { let type_alias_data = db . type_alias_signature (type_alias) ; let resolver = hir_def :: resolver :: HasResolver :: resolver (type_alias , db) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & type_alias_data . store , type_alias . into () , LifetimeElisionKind :: AnonymousReportError ,) ; let interner = ctx . interner ; let def_id = type_alias . into () ; let item_args = GenericArgs :: identity_for_item (interner , def_id) ; let interner_ty = Ty :: new_projection_from_args (interner , def_id , item_args) ; let mut bounds = Vec :: new () ; for bound in & type_alias_data . bounds { ctx . lower_type_bound (bound , interner_ty , false) . for_each (| pred | { bounds . push (pred) ; }) ; } if ! ctx . unsized_types . contains (& interner_ty) { let sized_trait = LangItem :: Sized . resolve_trait (ctx . db , interner . krate . expect ("Must have interner.krate")) ; if let Some (sized_trait) = sized_trait { let trait_ref = TraitRef :: new_from_args (interner , sized_trait . into () , GenericArgs :: new_from_iter (interner , [interner_ty . into ()]) ,) ; bounds . push (trait_ref . upcast (interner)) ; } ; } (EarlyBinder :: bind (bounds . into_boxed_slice ()) , create_diagnostics (ctx . diagnostics)) }
    };
}

type_alias_bounds_with_diagnostics!();