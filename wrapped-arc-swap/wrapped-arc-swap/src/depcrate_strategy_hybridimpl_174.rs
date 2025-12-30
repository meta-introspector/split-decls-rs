// Generated macro for impl_174 (impl)
macro_rules! Depcrate_strategy_hybridimpl_174 {
() => {
// Module: crate::strategy::hybrid
// Provides: {"impl_174"}
// Dependencies: {}
impl < T , Cfg > InnerStrategy < T > for HybridStrategy < Cfg > where T : RefCnt , Cfg : Config , { type Protected = HybridProtection < T > ; unsafe fn load (& self , storage : & AtomicPtr < T :: Base >) -> Self :: Protected { LocalNode :: with (| node | { let fast = if Cfg :: USE_FAST { HybridProtection :: attempt (node , storage) } else { None } ; fast . unwrap_or_else (| | HybridProtection :: fallback (node , storage)) }) } unsafe fn wait_for_readers (& self , old : * const T :: Base , storage : & AtomicPtr < T :: Base >) { let replacement = | | self . load (storage) . into_inner () ; Debt :: pay_all :: < T , _ > (old , storage as * const _ as usize , replacement) ; } }
};
}
