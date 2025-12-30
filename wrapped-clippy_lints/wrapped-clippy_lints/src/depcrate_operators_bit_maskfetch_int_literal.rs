// Generated macro for fetch_int_literal (function)
macro_rules! Depcrate_operators_bit_maskfetch_int_literal {
() => {
// Module: crate::operators::bit_mask
// Provides: {"fetch_int_literal"}
// Dependencies: {}
fn fetch_int_literal (cx : & LateContext < '_ > , lit : & Expr < '_ >) -> Option < u128 > { match ConstEvalCtxt :: new (cx) . eval (lit) ? { Constant :: Int (n) => Some (n) , _ => None , } }
};
}
