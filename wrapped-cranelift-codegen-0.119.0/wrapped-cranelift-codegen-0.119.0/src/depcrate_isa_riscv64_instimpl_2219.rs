// Generated macro for impl_2219 (impl)
macro_rules! Depcrate_isa_riscv64_instimpl_2219 {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"impl_2219"}
// Dependencies: {}
impl CondBrTarget { # [doc = " Return the target's label, if it is a label-based target."] pub (crate) fn as_label (self) -> Option < MachLabel > { match self { CondBrTarget :: Label (l) => Some (l) , _ => None , } } pub (crate) fn is_fallthrouh (& self) -> bool { self == & CondBrTarget :: Fallthrough } }
};
}
