// Generated macro for ToSqlOutput (enum)
macro_rules! Depcrate_types_to_sqlToSqlOutput {
() => {
// Module: crate::types::to_sql
// Provides: {"ToSqlOutput"}
// Dependencies: {}
# [doc = " `ToSqlOutput` represents the possible output types for implementers of the"] # [doc = " [`ToSql`] trait."] # [derive (Clone , Debug , PartialEq)] # [non_exhaustive] pub enum ToSqlOutput < 'a > { # [doc = " A borrowed SQLite-representable value."] Borrowed (ValueRef < 'a >) , # [doc = " An owned SQLite-representable value."] Owned (Value) , # [doc = " A BLOB of the given length that is filled with"] # [doc = " zeroes."] # [cfg (feature = "blob")] ZeroBlob (i32) , # [doc = " n-th arg of an SQL scalar function"] # [cfg (feature = "functions")] Arg (usize) , # [doc = " `feature = \"array\"`"] # [cfg (feature = "array")] Array (Array) , }
};
}
