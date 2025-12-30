// Generated macro for impl_469 (impl)
macro_rules! Depcrate_diagnostics_region_errorsimpl_469 {
() => {
// Module: crate::diagnostics::region_errors
// Provides: {"impl_469"}
// Dependencies: {}
impl < 'tcx > RegionErrors < 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx >) -> Self { Self (vec ! [] , tcx) } # [track_caller] pub (crate) fn push (& mut self , val : impl Into < RegionErrorKind < 'tcx > >) { let val = val . into () ; let guar = self . 1 . sess . dcx () . delayed_bug (format ! ("{val:?}")) ; self . 0 . push ((val , guar)) ; } pub (crate) fn is_empty (& self) -> bool { self . 0 . is_empty () } pub (crate) fn into_iter (self ,) -> impl Iterator < Item = (RegionErrorKind < 'tcx > , ErrorGuaranteed) > { self . 0 . into_iter () } }
};
}
