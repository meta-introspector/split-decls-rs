// Generated macro for impl_365 (impl)
macro_rules! Depcrate_machinst_regimpl_365 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_365"}
// Dependencies: {}
impl std :: convert :: From < regalloc2 :: VReg > for VirtualReg { fn from (vreg : regalloc2 :: VReg) -> VirtualReg { debug_assert ! (pinned_vreg_to_preg (vreg) . is_none ()) ; VirtualReg (vreg) } }
};
}
