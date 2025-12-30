// Generated macro for impl_57 (impl)
macro_rules! Depcrate_devimpl_57 {
() => {
// Module: crate::dev
// Provides: {"impl_57"}
// Dependencies: {}
impl PrimeField for Scalar { type Repr = FieldBytes ; const MODULUS : & 'static str = "0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff" ; const NUM_BITS : u32 = 256 ; const CAPACITY : u32 = 255 ; const TWO_INV : Self = Self :: ZERO ; const MULTIPLICATIVE_GENERATOR : Self = Self :: ZERO ; const S : u32 = 4 ; const ROOT_OF_UNITY : Self = Self :: ZERO ; const ROOT_OF_UNITY_INV : Self = Self :: ZERO ; const DELTA : Self = Self :: ZERO ; fn from_repr (bytes : FieldBytes) -> CtOption < Self > { ScalarValue :: from_bytes (& bytes) . map (Self) } fn to_repr (& self) -> FieldBytes { self . 0 . to_bytes () } fn is_odd (& self) -> Choice { self . 0 . is_odd () } }
};
}
