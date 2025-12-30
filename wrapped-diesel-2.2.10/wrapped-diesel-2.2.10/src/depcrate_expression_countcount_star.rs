// Generated macro for count_star (function)
macro_rules! Depcrate_expression_countcount_star {
() => {
// Module: crate::expression::count
// Provides: {"count_star"}
// Dependencies: {}
# [doc = " Creates a SQL `COUNT(*)` expression"] # [doc = ""] # [doc = " For selecting the count of a query, and nothing else, you can just call"] # [doc = " [`count`](crate::query_dsl::QueryDsl::count())"] # [doc = " on the query instead."] # [doc = ""] # [doc = " As with most bare functions, this is not exported by default. You can import"] # [doc = " it specifically as `diesel::dsl::count_star`, or glob import"] # [doc = " `diesel::dsl::*`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " assert_eq!(Ok(2), users.select(count_star()).first(connection));"] # [doc = " # }"] # [doc = " ```"] pub fn count_star () -> CountStar { CountStar }
};
}
