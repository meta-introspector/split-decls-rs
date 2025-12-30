// Generated macro for impl_119 (impl)
macro_rules! Depcrate_features_impl_stdimpl_119 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_119"}
// Dependencies: {}
impl < Context , T > Decode < Context > for RwLock < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (RwLock :: new (t)) } }
};
}
