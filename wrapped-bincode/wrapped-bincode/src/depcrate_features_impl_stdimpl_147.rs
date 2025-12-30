// Generated macro for impl_147 (impl)
macro_rules! Depcrate_features_impl_stdimpl_147 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_147"}
// Dependencies: {}
impl < K , V , S > Encode for HashMap < K , V , S > where K : Encode , V : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for (k , v) in self . iter () { Encode :: encode (k , encoder) ? ; Encode :: encode (v , encoder) ? ; } Ok (()) } }
};
}
