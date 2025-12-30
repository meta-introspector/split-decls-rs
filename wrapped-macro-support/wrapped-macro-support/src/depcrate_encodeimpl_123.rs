// Generated macro for impl_123 (impl)
macro_rules! Depcrate_encodeimpl_123 {
() => {
// Module: crate::encode
// Provides: {"impl_123"}
// Dependencies: {}
impl Encode for LitOrExpr < '_ > { fn encode (& self , dst : & mut Encoder) { match self { LitOrExpr :: Expr (expr) => { dst . dst . push (EncodeChunk :: StrExpr ((* expr) . clone ())) ; } LitOrExpr :: Lit (s) => s . encode (dst) , } } }
};
}
