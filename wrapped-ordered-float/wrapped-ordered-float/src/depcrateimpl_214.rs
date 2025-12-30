// Generated macro for impl_214 (impl)
macro_rules! Depcrateimpl_214 {
() => {
// Module: crate
// Provides: {"impl_214"}
// Dependencies: {}
impl < T : PrimitiveFloat > Hash for NotNan < T > { fn hash < H : Hasher > (& self , hasher : & mut H) { self . 0 . canonical_bits () . hash (hasher) ; } }
};
}
