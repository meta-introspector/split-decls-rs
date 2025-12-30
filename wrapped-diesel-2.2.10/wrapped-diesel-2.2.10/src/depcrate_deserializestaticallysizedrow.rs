// Generated macro for StaticallySizedRow (trait)
macro_rules! Depcrate_deserializeStaticallySizedRow {
() => {
// Module: crate::deserialize
// Provides: {"StaticallySizedRow"}
// Dependencies: {}
# [doc = " A marker trait indicating that the corresponding type consumes a static at"] # [doc = " compile time known number of field"] # [doc = ""] # [doc = " There is normally no need to implement this trait. Diesel provides"] # [doc = " wild card impls for all types that implement `FromSql<ST, DB>` or `Queryable<ST, DB>`"] # [doc = " where the size of `ST` is known"] pub trait StaticallySizedRow < ST , DB : Backend > : FromSqlRow < ST , DB > { # [doc = " The number of fields that this type will consume."] const FIELD_COUNT : usize ; }
};
}
