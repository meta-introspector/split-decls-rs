// Generated macro for hash_internals (module)
macro_rules! Depcratehash_internals {
() => {
// Module: crate
// Provides: {"hash_internals"}
// Dependencies: {}
mod hash_internals { pub trait SealedTrait : Copy + num_traits :: float :: FloatCore { type Bits : core :: hash :: Hash ; const CANONICAL_NAN_BITS : Self :: Bits ; fn canonical_bits (self) -> Self :: Bits ; } impl SealedTrait for f32 { type Bits = u32 ; const CANONICAL_NAN_BITS : u32 = 0x7fc00000 ; fn canonical_bits (self) -> u32 { (self + 0.0) . to_bits () } } impl SealedTrait for f64 { type Bits = u64 ; const CANONICAL_NAN_BITS : u64 = 0x7ff8000000000000 ; fn canonical_bits (self) -> u64 { (self + 0.0) . to_bits () } } }
};
}
