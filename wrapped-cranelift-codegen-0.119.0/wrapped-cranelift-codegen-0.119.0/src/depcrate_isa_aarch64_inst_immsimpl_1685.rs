// Generated macro for impl_1685 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1685 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1685"}
// Dependencies: {}
impl SImm7Scaled { # [doc = " Create a SImm7Scaled from a raw offset and the known scale type, if"] # [doc = " possible."] pub fn maybe_from_i64 (value : i64 , scale_ty : Type) -> Option < SImm7Scaled > { assert ! (scale_ty == I64 || scale_ty == I32 || scale_ty == F64 || scale_ty == I8X16) ; let scale = scale_ty . bytes () ; assert ! (scale . is_power_of_two ()) ; let scale = i64 :: from (scale) ; let upper_limit = 63 * scale ; let lower_limit = - (64 * scale) ; if value >= lower_limit && value <= upper_limit && (value & (scale - 1)) == 0 { Some (SImm7Scaled { value : i16 :: try_from (value) . unwrap () , scale_ty , }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { let ty_bytes : i16 = self . scale_ty . bytes () as i16 ; let scaled : i16 = self . value / ty_bytes ; assert ! (scaled <= 63 && scaled >= - 64) ; let scaled : i8 = scaled as i8 ; let encoded : u32 = scaled as u32 ; encoded & 0x7f } }
};
}
