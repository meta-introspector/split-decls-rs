// Generated macro for eval_lit (function)
macro_rules! Depcrate_interpeval_lit {
() => {
// Module: crate::interp
// Provides: {"eval_lit"}
// Dependencies: {}
# [doc = " Interprets a literal."] fn eval_lit (lit : & syn :: ExprLit) -> Option < u128 > { match & lit . lit { L :: Int (lit) => eval_lit_int (lit) , L :: Byte (lit) => Some (u128 :: from (lit . value ())) , L :: Verbatim (lit) => eval_lit_verbatim (lit) , _ => None , } }
};
}
