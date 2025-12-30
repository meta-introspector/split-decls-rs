// Generated macro for impl_156 (impl)
macro_rules! Depcrate_vecimpl_156 {
() => {
// Module: crate::vec
// Provides: {"impl_156"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < S : Encoder , I : Idx , T : Encodable < S > > Encodable < S > for IndexVec < I , T > { fn encode (& self , s : & mut S) { Encodable :: encode (& self . raw , s) ; } }
};
}
