// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl Hash for FixedBitSet { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . length . hash (state) ; self . as_simd_slice () . hash (state) ; } }
};
}
