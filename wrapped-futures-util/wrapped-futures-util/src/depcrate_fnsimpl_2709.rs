// Generated macro for impl_2709 (impl)
macro_rules! Depcrate_fnsimpl_2709 {
() => {
// Module: crate::fns
// Provides: {"impl_2709"}
// Dependencies: {}
impl < F , G , A > FnOnce1 < A > for ChainFn < F , G > where F : FnOnce1 < A > , G : FnOnce1 < F :: Output > , { type Output = G :: Output ; fn call_once (self , arg : A) -> Self :: Output { self . 1 . call_once (self . 0 . call_once (arg)) } }
};
}
