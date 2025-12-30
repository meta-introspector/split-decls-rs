// Generated macro for impl_62 (impl)
macro_rules! Depcrate_features_impl_allocimpl_62 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_62"}
// Dependencies: {}
impl < Context , T > Decode < Context > for VecDeque < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: decode (decoder) ? . into ()) } }
};
}
