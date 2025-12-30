// Generated macro for impl_2710 (impl)
macro_rules! Depcrate_fnsimpl_2710 {
() => {
// Module: crate::fns
// Provides: {"impl_2710"}
// Dependencies: {}
impl < F , G , A > FnMut1 < A > for ChainFn < F , G > where F : FnMut1 < A > , G : FnMut1 < F :: Output > , { fn call_mut (& mut self , arg : A) -> Self :: Output { self . 1 . call_mut (self . 0 . call_mut (arg)) } }
};
}
