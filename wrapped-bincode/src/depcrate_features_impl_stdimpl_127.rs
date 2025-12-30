// Generated macro for impl_127 (impl)
macro_rules! Depcrate_features_impl_stdimpl_127 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_127"}
// Dependencies: {}
impl < Context > Decode < Context > for PathBuf { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let string = std :: string :: String :: decode (decoder) ? ; Ok (string . into ()) } }
};
}
