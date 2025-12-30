// Generated macro for impl_53 (impl)
macro_rules! Depcrate_features_impl_allocimpl_53 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_53"}
// Dependencies: {}
impl < Context , T > Decode < Context > for BinaryHeap < T > where T : Decode < Context > + Ord , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: decode (decoder) ? . into ()) } }
};
}
