// Generated macro for EscapeExpressionMethods (trait)
macro_rules! Depcrate_expression_methods_escape_expression_methodsEscapeExpressionMethods {
() => {
// Module: crate::expression_methods::escape_expression_methods
// Provides: {"EscapeExpressionMethods"}
// Dependencies: {}
# [doc = " Adds the `escape` method to `LIKE` and `NOT LIKE`. This is used to specify"] # [doc = " the escape character for the pattern."] # [doc = ""] # [doc = " By default, the escape character is `\\` on most backends. On SQLite,"] # [doc = " there is no default escape character."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     use diesel::insert_into;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " #     insert_into(users).values(name.eq(\"Ha%%0r\"))"] # [doc = " #         .execute(connection).unwrap();"] # [doc = " let users_with_percent = users.select(name)"] # [doc = "     .filter(name.like(\"%😀%%\").escape('😀'))"] # [doc = "     .load(connection);"] # [doc = " let users_without_percent = users.select(name)"] # [doc = "     .filter(name.not_like(\"%a%%\").escape('a'))"] # [doc = "     .load(connection);"] # [doc = " assert_eq!(Ok(vec![String::from(\"Ha%%0r\")]), users_with_percent);"] # [doc = " assert_eq!(Ok(vec![String::from(\"Sean\"), String::from(\"Tess\")]), users_without_percent);"] # [doc = " # }"] # [doc = " ```"] pub trait EscapeExpressionMethods : Sized { # [doc (hidden)] type TextExpression ; # [doc = " See the trait documentation."] fn escape (self , _character : char) -> dsl :: Escape < Self > ; }
};
}
