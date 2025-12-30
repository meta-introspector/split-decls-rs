// Generated macro for impl_2711 (impl)
macro_rules! Depcrate_fnsimpl_2711 {
() => {
// Module: crate::fns
// Provides: {"impl_2711"}
// Dependencies: {}
impl < F , G , A > Fn1 < A > for ChainFn < F , G > where F : Fn1 < A > , G : Fn1 < F :: Output > , { fn call (& self , arg : A) -> Self :: Output { self . 1 . call (self . 0 . call (arg)) } }
};
}
