// Generated macro for NamedRow (trait)
macro_rules! Depcrate_rowNamedRow {
() => {
// Module: crate::row
// Provides: {"NamedRow"}
// Dependencies: {}
# [doc = " Represents a row of a SQL query, where the values are accessed by name"] # [doc = " rather than by index."] # [doc = ""] # [doc = " This trait is used by implementations of"] # [doc = " [`QueryableByName`](crate::deserialize::QueryableByName)"] pub trait NamedRow < 'a , DB : Backend > : Row < 'a , DB > { # [doc = " Retrieve and deserialize a single value from the query"] # [doc = ""] # [doc = " Note that `ST` *must* be the exact type of the value with that name in"] # [doc = " the query. The compiler will not be able to verify that you have"] # [doc = " provided the correct type. If there is a mismatch, you may receive an"] # [doc = " incorrect value, or a runtime error."] # [doc = ""] # [doc = " If two or more fields in the query have the given name, the result of"] # [doc = " this function is undefined."] fn get < ST , T > (& self , column_name : & str) -> deserialize :: Result < T > where T : FromSql < ST , DB > ; }
};
}
