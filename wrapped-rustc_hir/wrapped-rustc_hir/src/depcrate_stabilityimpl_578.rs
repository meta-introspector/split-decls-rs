// Generated macro for impl_578 (impl)
macro_rules! Depcrate_stabilityimpl_578 {
() => {
// Module: crate::stability
// Provides: {"impl_578"}
// Dependencies: {}
impl ConstStability { pub fn from_partial (PartialConstStability { level , feature , promotable } : PartialConstStability , const_stable_indirect : bool ,) -> Self { Self { const_stable_indirect , level , feature , promotable } } # [doc = " The stability assigned to unmarked items when -Zforce-unstable-if-unmarked is set."] pub fn unmarked (const_stable_indirect : bool , regular_stab : Stability) -> Self { Self { feature : regular_stab . feature , promotable : false , level : regular_stab . level , const_stable_indirect , } } pub fn is_const_unstable (& self) -> bool { self . level . is_unstable () } pub fn is_const_stable (& self) -> bool { self . level . is_stable () } }
};
}
