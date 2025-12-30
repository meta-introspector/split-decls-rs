// Generated macro for impl_1689 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1689 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1689"}
// Dependencies: {}
impl FPURightShiftImm { # [doc = " Create a floating-point unit immediate right shift from u8."] pub fn maybe_from_u8 (amount : u8 , lane_size_in_bits : u8) -> Option < Self > { debug_assert ! (lane_size_in_bits == 32 || lane_size_in_bits == 64) ; if amount > 0 && amount <= lane_size_in_bits { Some (Self { amount , lane_size_in_bits , }) } else { None } } # [doc = " Returns encoding of the immediate."] pub fn enc (& self) -> u32 { debug_assert_ne ! (0 , self . amount) ; u32 :: from ((self . lane_size_in_bits * 2) - self . amount) } }
};
}
