// Generated macro for ResultDataError (trait)
macro_rules! Depcrate_errorResultDataError {
() => {
// Module: crate::error
// Provides: {"ResultDataError"}
// Dependencies: {}
# [doc = " Extension trait for `Result<T, DataError>`."] pub trait ResultDataError < T > : Sized { # [doc = " Propagates all errors other than [`DataErrorKind::IdentifierNotFound`], and returns `None` in that case."] fn allow_identifier_not_found (self) -> Result < Option < T > , DataError > ; }
};
}
