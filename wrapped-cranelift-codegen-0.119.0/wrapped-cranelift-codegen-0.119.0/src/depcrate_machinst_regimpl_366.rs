// Generated macro for impl_366 (impl)
macro_rules! Depcrate_machinst_regimpl_366 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_366"}
// Dependencies: {}
impl std :: convert :: From < Reg > for regalloc2 :: VReg { # [doc = " Extract the underlying `regalloc2::VReg`. Note that physical"] # [doc = " registers also map to particular (special) VRegs, so this"] # [doc = " method can be used either on virtual or physical `Reg`s."] fn from (reg : Reg) -> regalloc2 :: VReg { reg . 0 } }
};
}
