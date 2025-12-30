// Generated macro for check_zst (function)
macro_rules! Depcrate_errorcheck_zst {
() => {
// Module: crate::error
// Provides: {"check_zst"}
// Dependencies: {}
pub (crate) fn check_zst < T > () -> Result < () > { if size_of :: < T > () == 0 { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_ZST_FORBIDDEN)) ; } Ok (()) }
};
}
