// Generated macro for all (function)
macro_rules! Depcrate_pg_expression_array_comparisonall {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"all"}
// Dependencies: {}
# [doc = " Creates a PostgreSQL `ALL` expression."] # [doc = ""] # [doc = " As with most bare functions, this is not exported by default. You can import"] # [doc = " it specifically as `diesel::pg::expression::dsl::all`, or `diesel::dsl::all`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " #     diesel::sql_query(\"INSERT INTO users (name) VALUES ('Jim')\").execute(connection).unwrap();"] # [doc = " let tess = (2, \"Tess\".to_string());"] # [doc = " let data = users.filter(name.ne(all(vec![\"Sean\", \"Jim\"])));"] # [doc = " assert_eq!(Ok(vec![tess]), data.load(connection));"] # [doc = " # }"] # [doc = " ```"] # [deprecated (since = "2.0.0" , note = "Use `ExpressionMethods::ne_all` instead")] pub fn all < ST , T > (vals : T) -> All < T :: Expression > where T : AsArrayExpression < ST > , { All :: new (vals . as_expression ()) }
};
}
