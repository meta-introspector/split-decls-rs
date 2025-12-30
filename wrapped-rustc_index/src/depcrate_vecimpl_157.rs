// Generated macro for impl_157 (impl)
macro_rules! Depcrate_vecimpl_157 {
() => {
// Module: crate::vec
// Provides: {"impl_157"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < D : Decoder , I : Idx , T : Decodable < D > > Decodable < D > for IndexVec < I , T > { fn decode (d : & mut D) -> Self { IndexVec :: from_raw (Vec :: < T > :: decode (d)) } }
};
}
