// Generated macro for macro_233 (macro)
macro_rules! Depcrate_expression_functions_aggregate_orderingmacro_233 {
() => {
// Module: crate::expression::functions::aggregate_ordering
// Provides: {"macro_233"}
// Dependencies: {}
define_sql_function ! { # [doc = " Represents a SQL `MAX` function. This function can only take types which are"] # [doc = " ordered."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::animals::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " assert_eq!(Ok(Some(8)), animals.select(max(legs)).first(connection));"] # [doc = " # }"] # [aggregate] fn max < ST : SqlOrdAggregate > (expr : ST) -> ST :: Ret ; }
};
}
