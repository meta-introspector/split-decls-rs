// Generated macro for joinable_inner (macro)
macro_rules! Depcrate_macrosjoinable_inner {
() => {
// Module: crate::macros
// Provides: {"joinable_inner"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! joinable_inner { ($ left_table : path => $ right_table : path : ($ foreign_key : path = $ parent_table : path)) => { $ crate :: joinable_inner ! (left_table_ty = $ left_table , right_table_ty = $ right_table , right_table_expr = $ right_table , foreign_key = $ foreign_key , primary_key_ty = <$ parent_table as $ crate :: query_source :: Table >:: PrimaryKey , primary_key_expr = <$ parent_table as $ crate :: query_source :: Table >:: primary_key (&$ parent_table) ,) ; } ; (left_table_ty = $ left_table_ty : ty , right_table_ty = $ right_table_ty : ty , right_table_expr = $ right_table_expr : expr , foreign_key = $ foreign_key : path , primary_key_ty = $ primary_key_ty : ty , primary_key_expr = $ primary_key_expr : expr ,) => { impl $ crate :: JoinTo <$ right_table_ty > for $ left_table_ty { type FromClause = $ right_table_ty ; type OnClause = $ crate :: dsl :: Eq < $ crate :: internal :: table_macro :: NullableExpression <$ foreign_key >, $ crate :: internal :: table_macro :: NullableExpression <$ primary_key_ty >, >; fn join_target (rhs : $ right_table_ty) -> (Self :: FromClause , Self :: OnClause) { use $ crate :: { ExpressionMethods , NullableExpressionMethods } ; (rhs , $ foreign_key . nullable () . eq ($ primary_key_expr . nullable ()) ,) } } } ; }
};
}
