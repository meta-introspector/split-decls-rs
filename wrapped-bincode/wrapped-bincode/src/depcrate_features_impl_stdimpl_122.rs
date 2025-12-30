// Generated macro for impl_122 (impl)
macro_rules! Depcrate_features_impl_stdimpl_122 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_122"}
// Dependencies: {}
impl < Context > Decode < Context > for SystemTime { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let duration = Duration :: decode (decoder) ? ; match SystemTime :: UNIX_EPOCH . checked_add (duration) { Some (t) => Ok (t) , None => Err (DecodeError :: InvalidSystemTime { duration }) , } } }
};
}
