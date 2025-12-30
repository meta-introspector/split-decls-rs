// Generated macro for index_consumed_at (function)
macro_rules! Depcrate_loops_char_indices_as_byte_indicesindex_consumed_at {
() => {
// Module: crate::loops::char_indices_as_byte_indices
// Provides: {"index_consumed_at"}
// Dependencies: {}
# [doc = " Returns the expression which ultimately consumes the index."] # [doc = " This is usually the parent expression, i.e. `.split_at(idx)` for `idx`,"] # [doc = " but for `.get(..idx)` we want to consider the method call the consuming expression,"] # [doc = " which requires skipping past the range expression."] fn index_consumed_at < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { for (_ , node) in cx . tcx . hir_parent_iter (expr . hir_id) { match node { Node :: Expr (expr) if higher :: Range :: hir (cx , expr) . is_some () => { } , Node :: ExprField (_) => { } , Node :: Expr (expr) => return Some (expr) , _ => break , } } None }
};
}
