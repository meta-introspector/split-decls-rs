// Generated macro for impl_213 (impl)
macro_rules! Depcrateimpl_213 {
() => {
// Module: crate
// Provides: {"impl_213"}
// Dependencies: {}
impl < T : PrimitiveFloat > Hash for OrderedFloat < T > { fn hash < H : Hasher > (& self , hasher : & mut H) { let bits = if self . 0 . is_nan () { T :: CANONICAL_NAN_BITS } else { self . 0 . canonical_bits () } ; bits . hash (hasher) ; } }
};
}
