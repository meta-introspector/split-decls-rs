// Generated macro for impl_869 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_869 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_869"}
// Dependencies: {}
impl < T : QuerySource , U > DeleteStatement < T , U , NoReturningClause > { # [doc = " Specify what expression is returned after execution of the `delete`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ### Deleting a record:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # #[cfg(feature = \"postgres\")]"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let deleted_name = diesel::delete(users.filter(name.eq(\"Sean\")))"] # [doc = "     .returning(name)"] # [doc = "     .get_result(connection);"] # [doc = " assert_eq!(Ok(\"Sean\".to_string()), deleted_name);"] # [doc = " # }"] # [doc = " # #[cfg(not(feature = \"postgres\"))]"] # [doc = " # fn main() {}"] # [doc = " ```"] pub fn returning < E > (self , returns : E) -> DeleteStatement < T , U , ReturningClause < E > > where E : SelectableExpression < T > , DeleteStatement < T , U , ReturningClause < E > > : Query , { DeleteStatement { where_clause : self . where_clause , from_clause : self . from_clause , returning : ReturningClause (returns) , } } }
};
}
