// Generated macro for constant_length (function)
macro_rules! Depcrate_manual_stripconstant_length {
() => {
// Module: crate::manual_strip
// Provides: {"constant_length"}
// Dependencies: {}
fn constant_length (cx : & LateContext < '_ > , expr : & Expr < '_ > , ctxt : SyntaxContext) -> Option < u128 > { let value = ConstEvalCtxt :: new (cx) . eval_local (expr , ctxt) ? ; match value { Constant :: Str (value) => Some (value . len () as u128) , Constant :: Char (value) => Some (value . len_utf8 () as u128) , _ => None , } }
};
}
