// Generated macro for impl_1687 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1687 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1687"}
// Dependencies: {}
impl FPULeftShiftImm { # [doc = " Create a floating-point unit immediate left shift from u8."] pub fn maybe_from_u8 (amount : u8 , lane_size_in_bits : u8) -> Option < Self > { debug_assert ! (lane_size_in_bits == 32 || lane_size_in_bits == 64) ; if amount < lane_size_in_bits { Some (Self { amount , lane_size_in_bits , }) } else { None } } # [doc = " Returns the encoding of the immediate."] pub fn enc (& self) -> u32 { debug_assert ! (self . lane_size_in_bits . is_power_of_two ()) ; debug_assert ! (self . lane_size_in_bits > self . amount) ; u32 :: from (self . lane_size_in_bits | self . amount) } }
};
}
