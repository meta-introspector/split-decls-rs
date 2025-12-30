// Generated macro for impl_284 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_284 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'tcx , T > TypeOpInfo < 'tcx > for DeeplyNormalizeQuery < 'tcx , T > where T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx , { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : Some (HigherRankedErrorCause :: CouldNotNormalize { value : self . canonical_query . canonical . value . value . value . to_string () , }) , span , }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; let ty :: ParamEnvAnd { param_env , value } = key ; let _ = ocx . deeply_normalize (& cause , param_env , value . value) ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }
};
}
