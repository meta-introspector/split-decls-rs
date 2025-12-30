// Generated macro for impl_714 (impl)
macro_rules! Depcrate_numberimpl_714 {
() => {
// Module: crate::number
// Provides: {"impl_714"}
// Dependencies: {}
# [cfg (not (feature = "arbitrary_precision"))] impl Hash for N { fn hash < H : Hasher > (& self , h : & mut H) { match * self { N :: PosInt (i) => i . hash (h) , N :: NegInt (i) => i . hash (h) , N :: Float (f) => { if f == 0.0f64 { 0.0f64 . to_bits () . hash (h) ; } else { f . to_bits () . hash (h) ; } } } } }
};
}
