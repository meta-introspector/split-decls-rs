// Generated macro for macro_228 (macro)
macro_rules! Depcrate_expression_functions_aggregate_foldingmacro_228 {
() => {
// Module: crate::expression::functions::aggregate_folding
// Provides: {"macro_228"}
// Dependencies: {}
define_sql_function ! { # [doc = " Represents a SQL `SUM` function. This function can only take types which are"] # [doc = " Foldable."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::animals::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " assert_eq!(Ok(Some(12i64)), animals.select(sum(legs)).first(connection));"] # [doc = " # }"] # [doc = " ```"] # [aggregate] fn sum < ST : Foldable > (expr : ST) -> ST :: Sum ; }
};
}
