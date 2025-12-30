// Generated macro for macro_234 (macro)
macro_rules! Depcrate_expression_functions_aggregate_orderingmacro_234 {
() => {
// Module: crate::expression::functions::aggregate_ordering
// Provides: {"macro_234"}
// Dependencies: {}
define_sql_function ! { # [doc = " Represents a SQL `MIN` function. This function can only take types which are"] # [doc = " ordered."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::animals::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " assert_eq!(Ok(Some(4)), animals.select(min(legs)).first(connection));"] # [doc = " # }"] # [aggregate] fn min < ST : SqlOrdAggregate > (expr : ST) -> ST :: Ret ; }
};
}
