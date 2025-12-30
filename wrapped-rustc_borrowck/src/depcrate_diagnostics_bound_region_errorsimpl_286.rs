// Generated macro for impl_286 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_286 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'tcx > TypeOpInfo < 'tcx > for AscribeUserTypeQuery < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : None , span }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; type_op_ascribe_user_type_with_span (& ocx , key , cause . span) . ok () ? ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }
};
}
