// Generated macro for no_arg_sql_function_body_except_to_sql (macro)
macro_rules! Depcrate_expression_functionsno_arg_sql_function_body_except_to_sql {
() => {
// Module: crate::expression::functions
// Provides: {"no_arg_sql_function_body_except_to_sql"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] macro_rules ! no_arg_sql_function_body_except_to_sql { ($ type_name : ident , $ return_type : ty , $ docs : expr) => { # [allow (non_camel_case_types)] # [doc =$ docs] # [derive (Debug , Clone , Copy , $ crate :: query_builder :: QueryId , $ crate :: expression :: ValidGrouping ,)] pub struct $ type_name ; impl $ crate :: expression :: Expression for $ type_name { type SqlType = $ return_type ; } impl < QS > $ crate :: expression :: SelectableExpression < QS > for $ type_name { } impl < QS > $ crate :: expression :: AppearsOnTable < QS > for $ type_name { } } ; }
};
}
