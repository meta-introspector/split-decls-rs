// Generated macro for impl_96 (impl)
macro_rules! Depcrate_iterators_pairimpl_96 {
() => {
// Module: crate::iterators::pair
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'i , R : Hash > Hash for Pair < 'i , R > { fn hash < H : Hasher > (& self , state : & mut H) { (& * self . queue as * const Vec < QueueableToken < 'i , R > >) . hash (state) ; (self . input as * const str) . hash (state) ; self . start . hash (state) ; } }
};
}
