// Generated macro for macro_363 (macro)
macro_rules! Depcrate_expression_countmacro_363 {
() => {
// Module: crate::expression::count
// Provides: {"macro_363"}
// Dependencies: {}
define_sql_function ! { # [doc = " Creates a SQL `COUNT` expression"] # [doc = ""] # [doc = " As with most bare functions, this is not exported by default. You can import"] # [doc = " it specifically as `diesel::dsl::count`, or glob import"] # [doc = " `diesel::dsl::*`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " # use diesel::dsl::*;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     use schema::animals::dsl::*;"] # [doc = " #     let connection = &mut establish_connection();"] # [doc = " assert_eq!(Ok(1), animals.select(count(name)).first(connection));"] # [doc = " # }"] # [doc = " ```"] # [aggregate] fn count < T : SqlType + SingleValue > (expr : T) -> BigInt ; }
};
}
