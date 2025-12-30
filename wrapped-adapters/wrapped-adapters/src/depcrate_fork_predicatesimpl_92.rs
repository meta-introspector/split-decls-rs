// Generated macro for impl_92 (impl)
macro_rules! Depcrate_fork_predicatesimpl_92 {
() => {
// Module: crate::fork::predicates
// Provides: {"impl_92"}
// Dependencies: {}
impl ForkByErrorPredicate for MarkerNotFoundPredicate { const UNIT_ERROR : DataErrorKind = DataErrorKind :: MarkerNotFound ; # [inline] fn test (& self , _ : DataMarkerInfo , _ : Option < DataRequest > , err : DataError) -> bool { matches ! (err , DataError { kind : DataErrorKind :: MarkerNotFound , .. }) } }
};
}
