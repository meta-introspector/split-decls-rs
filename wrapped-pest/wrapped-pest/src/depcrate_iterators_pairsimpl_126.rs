// Generated macro for impl_126 (impl)
macro_rules! Depcrate_iterators_pairsimpl_126 {
() => {
// Module: crate::iterators::pairs
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'i , R : Hash > Hash for Pairs < 'i , R > { fn hash < H : Hasher > (& self , state : & mut H) { (& * self . queue as * const Vec < QueueableToken < 'i , R > >) . hash (state) ; (self . input as * const str) . hash (state) ; self . start . hash (state) ; self . end . hash (state) ; } }
};
}
