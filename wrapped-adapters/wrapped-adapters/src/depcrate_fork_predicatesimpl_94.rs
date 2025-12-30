// Generated macro for impl_94 (impl)
macro_rules! Depcrate_fork_predicatesimpl_94 {
() => {
// Module: crate::fork::predicates
// Provides: {"impl_94"}
// Dependencies: {}
impl ForkByErrorPredicate for IdentifierNotFoundPredicate { const UNIT_ERROR : DataErrorKind = DataErrorKind :: IdentifierNotFound ; # [inline] fn test (& self , _ : DataMarkerInfo , _ : Option < DataRequest > , err : DataError) -> bool { Err :: < () , _ > (err) . allow_identifier_not_found () . is_ok () } }
};
}
