// Generated macro for impl_61 (impl)
macro_rules! Depcrate_features_impl_allocimpl_61 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > Encode for BTreeSet < T > where T : Encode + Ord , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } }
};
}
