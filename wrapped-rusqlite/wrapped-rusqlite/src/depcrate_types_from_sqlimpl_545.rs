// Generated macro for impl_545 (impl)
macro_rules! Depcrate_types_from_sqlimpl_545 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_545"}
// Dependencies: {}
impl FromSqlError { # [doc = " Converts an arbitrary error type to [`FromSqlError`]."] # [doc = ""] # [doc = " This is a convenience function that boxes and unsizes the error type. It's main purpose is"] # [doc = " to be usable in the `map_err` method. So instead of"] # [doc = " `result.map_err(|error| FromSqlError::Other(Box::new(error))` you can write"] # [doc = " `result.map_err(FromSqlError::other)`."] pub fn other < E : Error + Send + Sync + 'static > (error : E) -> Self { Self :: Other (Box :: new (error)) } }
};
}
