// Generated macro for impl_270 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_270 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'tcx > UniverseInfo < 'tcx > { pub (crate) fn other () -> UniverseInfo < 'tcx > { UniverseInfo :: Other } pub (crate) fn relate (expected : Ty < 'tcx > , found : Ty < 'tcx >) -> UniverseInfo < 'tcx > { UniverseInfo :: RelateTys { expected , found } } pub (crate) fn report_erroneous_element (& self , mbcx : & mut MirBorrowckCtxt < '_ , '_ , 'tcx > , placeholder : ty :: PlaceholderRegion , error_element : RegionElement , cause : ObligationCause < 'tcx > ,) { match * self { UniverseInfo :: RelateTys { expected , found } => { let err = mbcx . infcx . err_ctxt () . report_mismatched_types (& cause , mbcx . infcx . param_env , expected , found , TypeError :: RegionsPlaceholderMismatch ,) ; mbcx . buffer_error (err) ; } UniverseInfo :: TypeOp (ref type_op_info) => { type_op_info . report_erroneous_element (mbcx , placeholder , error_element , cause) ; } UniverseInfo :: Other => { mbcx . buffer_error (mbcx . dcx () . create_err (HigherRankedSubtypeError { span : cause . span }) ,) ; } } } }
};
}
