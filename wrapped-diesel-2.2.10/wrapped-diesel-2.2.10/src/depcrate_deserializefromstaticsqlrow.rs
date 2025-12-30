// Generated macro for FromStaticSqlRow (trait)
macro_rules! Depcrate_deserializeFromStaticSqlRow {
() => {
// Module: crate::deserialize
// Provides: {"FromStaticSqlRow"}
// Dependencies: {}
# [doc = " A helper trait to deserialize a statically sized row into a tuple"] # [doc = ""] # [doc = " **If you see an error message mentioning this trait you are likely trying to"] # [doc = " map the result of a query to a struct with mismatching field types. Recheck"] # [doc = " your field order and the concrete field types.**"] # [doc = ""] # [doc = " You should not need to implement this trait directly."] # [doc = " Diesel provides wild card implementations for any supported tuple size"] # [doc = " and for any type that implements `FromSql<ST, DB>`."] # [doc = ""] pub trait FromStaticSqlRow < ST , DB : Backend > : Sized { # [doc = " See the trait documentation"] fn build_from_row < 'a > (row : & impl Row < 'a , DB >) -> Result < Self > ; }
};
}
