// Generated macro for MaybeNullableType (trait)
macro_rules! Depcrate_sql_typesMaybeNullableType {
() => {
// Module: crate::sql_types
// Provides: {"MaybeNullableType"}
// Dependencies: {}
# [doc = " A type level constructor for maybe nullable types"] # [doc = ""] # [doc = " Constructs either `Nullable<O>` (for `Self` == `is_nullable::IsNullable`)"] # [doc = " or `O` (for `Self` == `is_nullable::NotNull`)"] pub trait MaybeNullableType < O > { # [doc = " See the trait documentation"] type Out : SqlType + TypedExpressionType ; }
};
}
