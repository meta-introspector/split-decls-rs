// Generated macro for impl_1750 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1750 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1750"}
// Dependencies: {}
impl OperandSize { # [doc = " 32-bit case?"] pub fn is32 (self) -> bool { self == OperandSize :: Size32 } # [doc = " 64-bit case?"] pub fn is64 (self) -> bool { self == OperandSize :: Size64 } # [doc = " Convert from a needed width to the smallest size that fits."] pub fn from_bits < I : Into < usize > > (bits : I) -> OperandSize { let bits : usize = bits . into () ; assert ! (bits <= 64) ; if bits <= 32 { OperandSize :: Size32 } else { OperandSize :: Size64 } } # [doc = " Return the operand size in bits."] pub fn bits (& self) -> u8 { match self { OperandSize :: Size32 => 32 , OperandSize :: Size64 => 64 , } } # [doc = " Convert from an integer type into the smallest size that fits."] pub fn from_ty (ty : Type) -> OperandSize { debug_assert ! (! ty . is_vector ()) ; Self :: from_bits (ty_bits (ty)) } # [doc = " Convert to I32, I64, or I128."] pub fn to_ty (self) -> Type { match self { OperandSize :: Size32 => I32 , OperandSize :: Size64 => I64 , } } # [doc = " Register interpretation bit."] # [doc = " When 0, the register is interpreted as the 32-bit version."] # [doc = " When 1, the register is interpreted as the 64-bit version."] pub fn sf_bit (& self) -> u32 { match self { OperandSize :: Size32 => 0 , OperandSize :: Size64 => 1 , } } # [doc = " The maximum unsigned value representable in a value of this size."] pub fn max_value (& self) -> u64 { match self { OperandSize :: Size32 => u32 :: MAX as u64 , OperandSize :: Size64 => u64 :: MAX , } } }
};
}
