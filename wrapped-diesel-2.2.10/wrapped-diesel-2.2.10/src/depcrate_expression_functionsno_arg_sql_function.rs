// Generated macro for no_arg_sql_function (macro)
macro_rules! Depcrate_expression_functionsno_arg_sql_function {
() => {
// Module: crate::expression::functions
// Provides: {"no_arg_sql_function"}
// Dependencies: {}
# [macro_export] # [doc = " Declare a 0 argument SQL function for use in your code. This will generate a"] # [doc = " unit struct, which is an expression representing calling this function. See"] # [doc = " [`now`](crate::expression::dsl::now) for example output. `now` was"] # [doc = " generated using:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # pub use diesel::*;"] # [doc = " no_arg_sql_function!(now, sql_types::Timestamp, \"Represents the SQL NOW() function\");"] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " You can optionally pass the name of a trait, as a constraint for backends which support the"] # [doc = " function."] # [deprecated (since = "2.0.0" , note = "Use `define_sql_function!` instead. See `CHANGELOG.md` for migration instructions")] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] macro_rules ! no_arg_sql_function { ($ type_name : ident , $ return_type : ty) => { no_arg_sql_function ! ($ type_name , $ return_type , "") ; } ; ($ type_name : ident , $ return_type : ty , $ docs : expr) => { no_arg_sql_function_body ! ($ type_name , $ return_type , $ docs) ; } ; ($ type_name : ident , $ return_type : ty , $ docs : expr , $ ($ constraint : ident) ::+) => { no_arg_sql_function_body ! ($ type_name , $ return_type , $ docs , $ ($ constraint) ::+) ; } ; }
};
}
