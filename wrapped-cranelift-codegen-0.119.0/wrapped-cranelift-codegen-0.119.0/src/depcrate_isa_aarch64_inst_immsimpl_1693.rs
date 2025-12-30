// Generated macro for impl_1693 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1693 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1693"}
// Dependencies: {}
impl UImm12Scaled { # [doc = " Create a UImm12Scaled from a raw offset and the known scale type, if"] # [doc = " possible."] pub fn maybe_from_i64 (value : i64 , scale_ty : Type) -> Option < UImm12Scaled > { let scale = scale_ty . bytes () ; assert ! (scale . is_power_of_two ()) ; let scale = scale as i64 ; let limit = 4095 * scale ; if value >= 0 && value <= limit && (value & (scale - 1)) == 0 { Some (UImm12Scaled { value : value as u16 , scale_ty , }) } else { None } } # [doc = " Create a zero immediate of this format."] pub fn zero (scale_ty : Type) -> UImm12Scaled { UImm12Scaled { value : 0 , scale_ty } } # [doc = " Encoded bits."] pub fn bits (& self) -> u32 { (self . value as u32 / self . scale_ty . bytes ()) & 0xfff } # [doc = " Value after scaling."] pub fn value (& self) -> u32 { self . value as u32 } }
};
}
