// Generated macro for impl_323 (impl)
macro_rules! Depcrate_machinst_valueregsimpl_323 {
() => {
// Module: crate::machinst::valueregs
// Provides: {"impl_323"}
// Dependencies: {}
impl < R : Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel > ValueRegs < R > { # [doc = " Create an invalid Value-in-Reg."] pub fn invalid () -> Self { ValueRegs { parts : [R :: invalid_sentinel () ; VALUE_REGS_PARTS] , } } # [doc = " Is this Value-to-Reg mapping valid?"] pub fn is_valid (self) -> bool { ! self . parts [0] . is_invalid_sentinel () } # [doc = " Is this Value-to-Reg mapping invalid?"] pub fn is_invalid (self) -> bool { self . parts [0] . is_invalid_sentinel () } # [doc = " Return the single register used for this value, if any."] pub fn only_reg (self) -> Option < R > { if self . len () == 1 { Some (self . parts [0]) } else { None } } # [doc = " Return a slice of the registers storing this value."] pub fn regs (& self) -> & [R] { & self . parts [0 .. self . len ()] } # [doc = " Return a mutable slice of the registers storing this value."] pub fn regs_mut (& mut self) -> & mut [R] { let len = self . len () ; & mut self . parts [0 .. len] } }
};
}
