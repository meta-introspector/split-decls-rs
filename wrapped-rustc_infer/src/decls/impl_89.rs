macro_rules! deps {
    () => {
        FreeRegionsVisitor!();
        VerifyIfEq!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'tcx , OP > TypeVisitor < TyCtxt < 'tcx > > for FreeRegionsVisitor < 'tcx , OP > where OP : FnMut (ty :: Region < 'tcx >) , { fn visit_region (& mut self , r : ty :: Region < 'tcx >) { match r . kind () { ty :: ReBound (_ , _) => { } _ => (self . op) (r) , } } fn visit_ty (& mut self , ty : Ty < 'tcx >) { if ! ty . flags () . intersects (ty :: TypeFlags :: HAS_FREE_REGIONS) { return ; } if ty . has_escaping_bound_vars () { return ty . super_visit_with (self) ; } match * ty . kind () { ty :: Alias (kind , ty :: AliasTy { def_id , args , .. }) => { let tcx = self . tcx ; let param_env = self . param_env ; let outlives_bounds : Vec < _ > = tcx . item_bounds (def_id) . iter_instantiated (tcx , args) . chain (param_env . caller_bounds ()) . filter_map (| clause | { let outlives = clause . as_type_outlives_clause () ? ; if let Some (outlives) = outlives . no_bound_vars () && outlives . 0 == ty { Some (outlives . 1) } else { test_type_match :: extract_verify_if_eq (tcx , & outlives . map_bound (| ty :: OutlivesPredicate (ty , bound) | { VerifyIfEq { ty , bound } }) , ty ,) } }) . collect () ; if outlives_bounds . contains (& tcx . lifetimes . re_static) { } else if let Some (r) = outlives_bounds . first () && outlives_bounds [1 ..] . iter () . all (| other_r | other_r == r) { assert ! (r . type_flags () . intersects (ty :: TypeFlags :: HAS_FREE_REGIONS)) ; r . visit_with (self) ; } else { let variances = tcx . opt_alias_variances (kind , def_id) ; for (idx , s) in args . iter () . enumerate () { if variances . map (| variances | variances [idx]) != Some (ty :: Bivariant) { s . visit_with (self) ; } } } } _ => ty . super_visit_with (self) , } } }
    };
}

impl_89!()