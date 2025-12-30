// Generated macro for const_str (function)
macro_rules! Depcrate_regexconst_str {
() => {
// Module: crate::regex
// Provides: {"const_str"}
// Dependencies: {}
fn const_str < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) -> Option < String > { ConstEvalCtxt :: new (cx) . eval (e) . and_then (| c | match c { Constant :: Str (s) => Some (s) , _ => None , }) }
};
}
