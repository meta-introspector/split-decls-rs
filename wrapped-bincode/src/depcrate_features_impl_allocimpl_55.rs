// Generated macro for impl_55 (impl)
macro_rules! Depcrate_features_impl_allocimpl_55 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_55"}
// Dependencies: {}
impl < T > Encode for BinaryHeap < T > where T : Encode + Ord , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for val in self . iter () { val . encode (encoder) ? ; } Ok (()) } }
};
}
