// Generated macro for numeric_operation (macro)
macro_rules! Depcrate_expression_ops_numericnumeric_operation {
() => {
// Module: crate::expression::ops::numeric
// Provides: {"numeric_operation"}
// Dependencies: {}
macro_rules ! numeric_operation { ($ name : ident , $ op : expr) => { # [doc (hidden)] # [derive (Debug , Copy , Clone , QueryId , ValidGrouping)] pub struct $ name < Lhs , Rhs > { lhs : Lhs , rhs : Rhs , } impl < Lhs , Rhs > $ name < Lhs , Rhs > { # [doc (hidden)] pub fn new (left : Lhs , right : Rhs) -> Self { $ name { lhs : left , rhs : right , } } } impl < Lhs , Rhs > Expression for $ name < Lhs , Rhs > where Lhs : Expression , Lhs :: SqlType : sql_types :: ops ::$ name , Rhs : Expression , < Lhs :: SqlType as sql_types :: ops ::$ name >:: Output : TypedExpressionType , { type SqlType = < Lhs :: SqlType as sql_types :: ops ::$ name >:: Output ; } impl < Lhs , Rhs , DB > QueryFragment < DB > for $ name < Lhs , Rhs > where DB : Backend , Lhs : QueryFragment < DB >, Rhs : QueryFragment < DB >, { fn walk_ast <'b > (&'b self , mut out : AstPass <'_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("(") ; self . lhs . walk_ast (out . reborrow ()) ?; out . push_sql ($ op) ; self . rhs . walk_ast (out . reborrow ()) ?; out . push_sql (")") ; Ok (()) } } impl_selectable_expression ! ($ name < Lhs , Rhs >) ; generic_numeric_expr ! ($ name , A , B) ; } ; }
};
}
