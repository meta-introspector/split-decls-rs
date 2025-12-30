// Generated macro for impl_18 (impl)
macro_rules! Depcrate_validatorimpl_18 {
() => {
// Module: crate::validator
// Provides: {"impl_18"}
// Dependencies: {}
impl From < serde_json :: Error > for ValidationError { fn from (err : serde_json :: Error) -> Self { ValidationError :: DataAccessError { message : format ! ("JSON serialization error: {}" , err) , } } }
};
}
