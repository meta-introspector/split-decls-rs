// Generated macro for SingleValue (trait)
macro_rules! Depcrate_sql_typesSingleValue {
() => {
// Module: crate::sql_types
// Provides: {"SingleValue"}
// Dependencies: {}
# [doc = " A marker trait indicating that a SQL type represents a single value, as"] # [doc = " opposed to a list of values."] # [doc = ""] # [doc = " This trait should generally be implemented for all SQL types with the"] # [doc = " exception of Rust tuples. If a column could have this as its type, this"] # [doc = " trait should be implemented."] # [doc = ""] # [doc = " # Deriving"] # [doc = ""] # [doc = " This trait is automatically implemented by [`#[derive(SqlType)]`](derive@SqlType)"] # [doc = ""] pub trait SingleValue : SqlType { }
};
}
