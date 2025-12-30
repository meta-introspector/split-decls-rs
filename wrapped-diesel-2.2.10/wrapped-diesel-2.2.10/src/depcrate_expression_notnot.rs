// Generated macro for not (function)
macro_rules! Depcrate_expression_notnot {
() => {
// Module: crate::expression::not
// Provides: {"not"}
// Dependencies: {}
# [doc = " Creates a SQL `NOT` expression"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " use diesel::dsl::not;"] # [doc = ""] # [doc = " let users_with_name = users.select(id).filter(name.eq(\"Sean\"));"] # [doc = " let users_not_with_name = users.select(id).filter("] # [doc = "     not(name.eq(\"Sean\")));"] # [doc = ""] # [doc = " assert_eq!(Ok(1), users_with_name.first(connection));"] # [doc = " assert_eq!(Ok(2), users_not_with_name.first(connection));"] # [doc = " # }"] # [doc = " ```"] pub fn not < T > (expr : T) -> helper_types :: not < T > where T : Expression , < T as Expression > :: SqlType : BoolOrNullableBool , { super :: operators :: Not :: new (Grouped (expr)) }
};
}
