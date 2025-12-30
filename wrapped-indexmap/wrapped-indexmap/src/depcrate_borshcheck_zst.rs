// Generated macro for check_zst (function)
macro_rules! Depcrate_borshcheck_zst {
() => {
// Module: crate::borsh
// Provides: {"check_zst"}
// Dependencies: {}
fn check_zst < T > () -> Result < () > { if size_of :: < T > () == 0 { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_ZST_FORBIDDEN)) ; } Ok (()) }
};
}
