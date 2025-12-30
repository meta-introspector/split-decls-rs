// Generated macro for generic_numeric_expr (macro)
macro_rules! Depcrate_expression_opsgeneric_numeric_expr {
() => {
// Module: crate::expression::ops
// Provides: {"generic_numeric_expr"}
// Dependencies: {}
macro_rules ! generic_numeric_expr { ($ tpe : ident , $ ($ param : ident) ,*) => { generic_numeric_expr_inner ! ($ tpe , ($ ($ param) ,*) , Add , add) ; generic_numeric_expr_inner ! ($ tpe , ($ ($ param) ,*) , Sub , sub) ; generic_numeric_expr_inner ! ($ tpe , ($ ($ param) ,*) , Div , div) ; generic_numeric_expr_inner ! ($ tpe , ($ ($ param) ,*) , Mul , mul) ; } }
};
}
