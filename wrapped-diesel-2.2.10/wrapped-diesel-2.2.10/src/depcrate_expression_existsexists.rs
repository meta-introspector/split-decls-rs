// Generated macro for exists (function)
macro_rules! Depcrate_expression_existsexists {
() => {
// Module: crate::expression::exists
// Provides: {"exists"}
// Dependencies: {}
# [doc = " Creates a SQL `EXISTS` expression."] # [doc = ""] # [doc = " The argument must be a complete SQL query. The query may reference columns"] # [doc = " from the outer table."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     use diesel::select;"] # [doc = " #     use diesel::dsl::exists;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let sean_exists = select(exists(users.filter(name.eq(\"Sean\"))))"] # [doc = "     .get_result(connection);"] # [doc = " let jim_exists = select(exists(users.filter(name.eq(\"Jim\"))))"] # [doc = "     .get_result(connection);"] # [doc = " assert_eq!(Ok(true), sean_exists);"] # [doc = " assert_eq!(Ok(false), jim_exists);"] # [doc = " # }"] # [doc = " ```"] pub fn exists < T > (query : T) -> helper_types :: exists < T > { Exists { subselect : Subselect :: new (query) , } }
};
}
