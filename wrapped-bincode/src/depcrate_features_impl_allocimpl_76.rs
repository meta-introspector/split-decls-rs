// Generated macro for impl_76 (impl)
macro_rules! Depcrate_features_impl_allocimpl_76 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_76"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Box < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into_boxed_slice ()) } }
};
}
