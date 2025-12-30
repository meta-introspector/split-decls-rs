// Generated macro for SqlOrd (trait)
macro_rules! Depcrate_sql_types_ordSqlOrd {
() => {
// Module: crate::sql_types::ord
// Provides: {"SqlOrd"}
// Dependencies: {}
# [doc = " Marker trait for types which can be used with `MAX` and `MIN`"] # [diagnostic :: on_unimplemented (message = "expressions of the type `{Self}` cannot be ordered by the database")] pub trait SqlOrd : SqlType { }
};
}
