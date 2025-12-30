// Generated macro for ensure_impl_params_and_item_params_correspond (function)
macro_rules! Depcrate_check_always_applicableensure_impl_params_and_item_params_correspond {
() => {
// Module: crate::check::always_applicable
// Provides: {"ensure_impl_params_and_item_params_correspond"}
// Dependencies: {}
fn ensure_impl_params_and_item_params_correspond < 'tcx > (tcx : TyCtxt < 'tcx > , impl_def_id : LocalDefId , adt_def_id : DefId , adt_to_impl_args : GenericArgsRef < 'tcx > ,) -> Result < () , ErrorGuaranteed > { let Err (arg) = tcx . uses_unique_generic_params (adt_to_impl_args , CheckRegions :: OnlyParam) else { return Ok (()) ; } ; let impl_span = tcx . def_span (impl_def_id) ; let item_span = tcx . def_span (adt_def_id) ; let self_descr = tcx . def_descr (adt_def_id) ; let polarity = match tcx . impl_polarity (impl_def_id) { ty :: ImplPolarity :: Positive | ty :: ImplPolarity :: Reservation => "" , ty :: ImplPolarity :: Negative => "!" , } ; let trait_name = tcx . item_name (tcx . trait_id_of_impl (impl_def_id . to_def_id ()) . expect ("expected impl of trait")) ; let mut err = struct_span_code_err ! (tcx . dcx () , impl_span , E0366 , "`{polarity}{trait_name}` impls cannot be specialized" ,) ; match arg { ty :: util :: NotUniqueParam :: DuplicateParam (arg) => { err . note (format ! ("`{arg}` is mentioned multiple times")) } ty :: util :: NotUniqueParam :: NotParam (arg) => { err . note (format ! ("`{arg}` is not a generic parameter")) } } ; err . span_note (item_span , format ! ("use the same sequence of generic lifetime, type and const parameters \
                     as the {self_descr} definition" ,) ,) ; Err (err . emit ()) }
};
}
