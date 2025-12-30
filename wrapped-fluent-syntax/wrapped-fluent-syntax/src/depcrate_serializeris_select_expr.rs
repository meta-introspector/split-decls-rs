// Generated macro for is_select_expr (function)
macro_rules! Depcrate_serializeris_select_expr {
() => {
// Module: crate::serializer
// Provides: {"is_select_expr"}
// Dependencies: {}
fn is_select_expr < 's , S : Slice < 's > > (expr : & Expression < S >) -> bool { match expr { Expression :: Select { .. } => true , Expression :: Inline (InlineExpression :: Placeable { expression }) => { is_select_expr (expression) } Expression :: Inline (_) => false , } }
};
}
