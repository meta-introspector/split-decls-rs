// Generated macro for impl_1699 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1699 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1699"}
// Dependencies: {}
impl ImmShift { # [doc = " Create an ImmShift from raw bits, if possible."] pub fn maybe_from_u64 (val : u64) -> Option < ImmShift > { if val < 64 { Some (ImmShift { imm : val as u8 }) } else { None } } # [doc = " Get the immediate value."] pub fn value (& self) -> u8 { self . imm } }
};
}
