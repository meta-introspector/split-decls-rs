// Generated macro for FromSqlRow (trait)
macro_rules! Depcrate_deserializeFromSqlRow {
() => {
// Module: crate::deserialize
// Provides: {"FromSqlRow"}
// Dependencies: {}
# [doc = " Deserialize a database row into a rust data structure"] # [doc = ""] # [doc = " Diesel provides wild card implementations of this trait for all types"] # [doc = " that implement one of the following traits:"] # [doc = "    * [`Queryable`]"] # [doc = "    * [`QueryableByName`]"] # [diagnostic :: on_unimplemented (note = "double check your type mappings via the documentation of `{ST}`" , note = "`diesel::sql_query` requires the loading target to column names for loading values.\n\
             You need to provide a type that explicitly derives `diesel::deserialize::QueryableByName`")] pub trait FromSqlRow < ST , DB : Backend > : Sized { # [doc = " See the trait documentation."] fn build_from_row < 'a > (row : & impl Row < 'a , DB >) -> Result < Self > ; }
};
}
