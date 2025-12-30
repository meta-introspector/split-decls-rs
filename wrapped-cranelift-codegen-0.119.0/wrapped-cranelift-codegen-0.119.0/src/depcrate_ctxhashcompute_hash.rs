// Generated macro for compute_hash (function)
macro_rules! Depcrate_ctxhashcompute_hash {
() => {
// Module: crate::ctxhash
// Provides: {"compute_hash"}
// Dependencies: {}
fn compute_hash < Ctx , K > (ctx : & Ctx , k : & K) -> u32 where Ctx : CtxHash < K > , { let mut hasher = rustc_hash :: FxHasher :: default () ; ctx . ctx_hash (& mut hasher , k) ; hasher . finish () as u32 }
};
}
