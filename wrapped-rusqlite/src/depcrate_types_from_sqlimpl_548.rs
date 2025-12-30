// Generated macro for impl_548 (impl)
macro_rules! Depcrate_types_from_sqlimpl_548 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_548"}
// Dependencies: {}
impl Error for FromSqlError { fn source (& self) -> Option < & (dyn Error + 'static) > { if let Self :: Other (ref err) = self { Some (& * * err) } else { None } } }
};
}
