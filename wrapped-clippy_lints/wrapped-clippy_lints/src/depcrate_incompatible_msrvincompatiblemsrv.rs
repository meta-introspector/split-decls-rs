// Generated macro for IncompatibleMsrv (struct)
macro_rules! Depcrate_incompatible_msrvIncompatibleMsrv {
() => {
// Module: crate::incompatible_msrv
// Provides: {"IncompatibleMsrv"}
// Dependencies: {}
pub struct IncompatibleMsrv { msrv : Msrv , availability_cache : FxHashMap < (DefId , bool) , Availability > , check_in_tests : bool , std_crates : StdCrates , called_path : Option < HirId > , }
};
}
