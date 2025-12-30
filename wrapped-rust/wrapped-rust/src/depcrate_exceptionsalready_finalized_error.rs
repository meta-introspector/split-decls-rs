// Generated macro for already_finalized_error (function)
macro_rules! Depcrate_exceptionsalready_finalized_error {
() => {
// Module: crate::exceptions
// Provides: {"already_finalized_error"}
// Dependencies: {}
pub (crate) fn already_finalized_error () -> CryptographyError { CryptographyError :: from (AlreadyFinalized :: new_err ("Context was already finalized.")) }
};
}
