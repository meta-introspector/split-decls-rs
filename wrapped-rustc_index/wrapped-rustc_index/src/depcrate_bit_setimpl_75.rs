// Generated macro for impl_75 (impl)
macro_rules! Depcrate_bit_setimpl_75 {
() => {
// Module: crate::bit_set
// Provides: {"impl_75"}
// Dependencies: {}
impl FiniteBitSetTy for u32 { const DOMAIN_SIZE : u32 = 32 ; const FILLED : Self = Self :: MAX ; const EMPTY : Self = Self :: MIN ; const ONE : Self = 1u32 ; const ZERO : Self = 0u32 ; fn checked_shl (self , rhs : u32) -> Option < Self > { self . checked_shl (rhs) } fn checked_shr (self , rhs : u32) -> Option < Self > { self . checked_shr (rhs) } }
};
}
