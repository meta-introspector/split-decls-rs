macro_rules! try_extract_error_from_fulfill_cx {
    () => {
        # [instrument (skip (ocx) , level = "debug")] fn try_extract_error_from_fulfill_cx < 'a , 'tcx > (ocx : & ObligationCtxt < 'a , 'tcx > , generic_param_scope : LocalDefId , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'a > > { let _errors = ocx . select_all_or_error () ; let region_constraints = ocx . infcx . with_region_constraints (| r | r . clone ()) ; try_extract_error_from_region_constraints (ocx . infcx , generic_param_scope , placeholder_region , error_region , & region_constraints , | vid | ocx . infcx . region_var_origin (vid) , | vid | ocx . infcx . universe_of_region (ty :: Region :: new_var (ocx . infcx . tcx , vid)) ,) }
    };
}

try_extract_error_from_fulfill_cx!()