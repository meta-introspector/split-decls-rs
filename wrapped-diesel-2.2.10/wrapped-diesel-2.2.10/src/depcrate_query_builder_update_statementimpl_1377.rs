// Generated macro for impl_1377 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1377 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1377"}
// Dependencies: {}
impl < T : QuerySource , U , V > UpdateStatement < T , U , V , NoReturningClause > { # [doc = " Specify what expression is returned after execution of the `update`."] # [doc = " # Examples"] # [doc = ""] # [doc = " ### Updating a single record:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # #[cfg(feature = \"postgres\")]"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let updated_name = diesel::update(users.filter(id.eq(1)))"] # [doc = "     .set(name.eq(\"Dean\"))"] # [doc = "     .returning(name)"] # [doc = "     .get_result(connection);"] # [doc = " assert_eq!(Ok(\"Dean\".to_string()), updated_name);"] # [doc = " # }"] # [doc = " # #[cfg(not(feature = \"postgres\"))]"] # [doc = " # fn main() {}"] # [doc = " ```"] pub fn returning < E > (self , returns : E) -> UpdateStatement < T , U , V , ReturningClause < E > > where T : Table , UpdateStatement < T , U , V , ReturningClause < E > > : Query , { UpdateStatement { from_clause : self . from_clause , where_clause : self . where_clause , values : self . values , returning : ReturningClause (returns) , } } }
};
}
