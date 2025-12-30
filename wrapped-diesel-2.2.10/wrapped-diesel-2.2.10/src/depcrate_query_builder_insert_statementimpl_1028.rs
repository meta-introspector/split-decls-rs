// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1028 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1028"}
// Dependencies: {}
impl < T : QuerySource , U , Op > InsertStatement < T , U , Op > { # [doc = " Specify what expression is returned after execution of the `insert`."] # [doc = " # Examples"] # [doc = ""] # [doc = " ### Inserting records:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # #[cfg(feature = \"postgres\")]"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let inserted_names = diesel::insert_into(users)"] # [doc = "     .values(&vec![name.eq(\"Timmy\"), name.eq(\"Jimmy\")])"] # [doc = "     .returning(name)"] # [doc = "     .get_results(connection)"] # [doc = "     .unwrap();"] # [doc = " // Note that the returned order is not guaranteed to be preserved"] # [doc = " assert_eq!(inserted_names.len(), 2);"] # [doc = " assert!(inserted_names.contains(&\"Timmy\".to_string()));"] # [doc = " assert!(inserted_names.contains(&\"Jimmy\".to_string()));"] # [doc = " # }"] # [doc = " # #[cfg(not(feature = \"postgres\"))]"] # [doc = " # fn main() {}"] # [doc = " ```"] pub fn returning < E > (self , returns : E) -> InsertStatement < T , U , Op , ReturningClause < E > > where InsertStatement < T , U , Op , ReturningClause < E > > : Query , { InsertStatement :: new (self . target , self . records , self . operator , ReturningClause (returns) ,) } }
};
}
