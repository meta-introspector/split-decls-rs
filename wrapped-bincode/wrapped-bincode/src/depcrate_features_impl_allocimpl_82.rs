// Generated macro for impl_82 (impl)
macro_rules! Depcrate_features_impl_allocimpl_82 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_82"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Rc < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Rc :: new (t)) } }
};
}
