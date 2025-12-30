// Generated macro for impl_83 (impl)
macro_rules! Depcrate_features_impl_allocimpl_83 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_83"}
// Dependencies: {}
impl < Context > Decode < Context > for Rc < str > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
};
}
