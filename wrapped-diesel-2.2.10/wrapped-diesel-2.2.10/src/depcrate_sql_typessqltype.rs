// Generated macro for SqlType (trait)
macro_rules! Depcrate_sql_typesSqlType {
() => {
// Module: crate::sql_types
// Provides: {"SqlType"}
// Dependencies: {}
# [doc = " A marker trait for SQL types"] # [doc = ""] # [doc = " # Deriving"] # [doc = ""] # [doc = " This trait is automatically implemented by [`#[derive(SqlType)]`](derive@SqlType)"] # [doc = " which sets `IsNull` to [`is_nullable::NotNull`]"] # [doc = ""] pub trait SqlType : 'static { # [doc = " Is this type nullable?"] # [doc = ""] # [doc = " This type should always be one of the structs in the ['is_nullable`]"] # [doc = " module. See the documentation of those structs for more details."] # [doc = ""] # [doc = " ['is_nullable`]: is_nullable"] type IsNull : OneIsNullable < is_nullable :: IsNullable > + OneIsNullable < is_nullable :: NotNull > ; }
};
}
