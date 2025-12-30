// Generated macro for impl_73 (impl)
macro_rules! Depcrate_features_impl_allocimpl_73 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_73"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Box < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Box :: new (t)) } }
};
}
