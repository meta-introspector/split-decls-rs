// Generated macro for extract_inner_functions (function)
macro_rules! Depcrate_testextract_inner_functions {
() => {
// Module: crate::test
// Provides: {"extract_inner_functions"}
// Dependencies: {}
pub (crate) fn extract_inner_functions (block : & syn :: Block) -> impl Iterator < Item = & syn :: ItemFn > { block . stmts . iter () . filter_map (| s | match s { syn :: Stmt :: Item (syn :: Item :: Fn (f)) => Some (f) , _ => None , }) }
};
}
