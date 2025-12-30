// Generated macro for no_arg_sql_function_body (macro)
macro_rules! Depcrate_expression_functionsno_arg_sql_function_body {
() => {
// Module: crate::expression::functions
// Provides: {"no_arg_sql_function_body"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] macro_rules ! no_arg_sql_function_body { ($ type_name : ident , $ return_type : ty , $ docs : expr , $ ($ constraint : ident) ::+) => { no_arg_sql_function_body_except_to_sql ! ($ type_name , $ return_type , $ docs) ; impl < DB > $ crate :: query_builder :: QueryFragment < DB > for $ type_name where DB : $ crate :: backend :: Backend + $ ($ constraint) ::+, { fn walk_ast <'b > (&'b self , mut out : $ crate :: query_builder :: AstPass <'_ , 'b , DB >) -> $ crate :: result :: QueryResult < () > { out . push_sql (concat ! (stringify ! ($ type_name) , "()")) ; Ok (()) } } } ; ($ type_name : ident , $ return_type : ty , $ docs : expr) => { no_arg_sql_function_body_except_to_sql ! ($ type_name , $ return_type , $ docs) ; impl < DB > $ crate :: query_builder :: QueryFragment < DB > for $ type_name where DB : $ crate :: backend :: Backend , { fn walk_ast <'b > (&'b self , mut out : $ crate :: query_builder :: AstPass <'_ , 'b , DB >) -> $ crate :: result :: QueryResult < () > { out . push_sql (concat ! (stringify ! ($ type_name) , "()")) ; Ok (()) } } } ; }
};
}
