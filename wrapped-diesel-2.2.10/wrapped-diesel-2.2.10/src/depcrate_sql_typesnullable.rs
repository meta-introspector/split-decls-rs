// Generated macro for Nullable (struct)
macro_rules! Depcrate_sql_typesNullable {
() => {
// Module: crate::sql_types
// Provides: {"Nullable"}
// Dependencies: {}
# [doc = " The nullable SQL type."] # [doc = ""] # [doc = " This wraps another SQL type to indicate that it can be null."] # [doc = " By default all values are assumed to be `NOT NULL`."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - Any `T` which implements `ToSql<ST>`"] # [doc = " - `Option<T>` for any `T` which implements `ToSql<ST>`"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - `Option<T>` for any `T` which implements `FromSql<ST>`"] # [derive (Debug , Clone , Copy , Default)] pub struct Nullable < ST > (ST) ;
};
}
