// Generated macro for impl_152 (impl)
macro_rules! Depcrate_features_impl_stdimpl_152 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_152"}
// Dependencies: {}
impl < T , S > Encode for HashSet < T , S > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } }
};
}
