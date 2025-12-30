// Generated macro for impl_353 (impl)
macro_rules! Depcrate_machinst_regimpl_353 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_353"}
// Dependencies: {}
impl Reg { # [doc = " Get the physical register (`RealReg`), if this register is"] # [doc = " one."] pub fn to_real_reg (self) -> Option < RealReg > { pinned_vreg_to_preg (self . 0) . map (RealReg) } # [doc = " Get the virtual (non-physical) register, if this register is"] # [doc = " one."] pub fn to_virtual_reg (self) -> Option < VirtualReg > { if pinned_vreg_to_preg (self . 0) . is_none () { Some (VirtualReg (self . 0)) } else { None } } # [doc = " Get the class of this register."] pub fn class (self) -> RegClass { self . 0 . class () } # [doc = " Is this a real (physical) reg?"] pub fn is_real (self) -> bool { self . to_real_reg () . is_some () } # [doc = " Is this a virtual reg?"] pub fn is_virtual (self) -> bool { self . to_virtual_reg () . is_some () } }
};
}
