// Generated macro for impl_116 (impl)
macro_rules! Depcrate_features_impl_stdimpl_116 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_116"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Mutex < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Mutex :: new (t)) } }
};
}
