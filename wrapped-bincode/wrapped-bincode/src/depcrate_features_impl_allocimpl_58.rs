// Generated macro for impl_58 (impl)
macro_rules! Depcrate_features_impl_allocimpl_58 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_58"}
// Dependencies: {}
impl < K , V > Encode for BTreeMap < K , V > where K : Encode + Ord , V : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for (key , val) in self . iter () { key . encode (encoder) ? ; val . encode (encoder) ? ; } Ok (()) } }
};
}
