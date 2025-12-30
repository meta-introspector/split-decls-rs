// Generated macro for macro_250 (macro)
macro_rules! Depcrate_expression_functions_date_and_timemacro_250 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"macro_250"}
// Dependencies: {}
define_sql_function ! { # [doc = " Represents the SQL `DATE` function. The argument should be a Timestamp"] # [doc = " expression, and the return value will be an expression of type Date."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # include!(\"../../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::{now, date};"] # [doc = " # use diesel::deserialize::Queryable;"] # [doc = " #"] # [doc = " # fn test<R: Queryable<diesel::sql_types::Date, DB> + 'static>() -> QueryResult<R> {"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " let today = diesel::select(date(now)).first(connection)?;"] # [doc = " #     Ok(today)"] # [doc = " # }"] # [doc = " # fn main() {"] # [doc = " #"] # [doc = " # }"] # [doc = " ```"] fn date (expr : Timestamp) -> Date ; }
};
}
