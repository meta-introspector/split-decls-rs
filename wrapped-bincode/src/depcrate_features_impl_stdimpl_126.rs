// Generated macro for impl_126 (impl)
macro_rules! Depcrate_features_impl_stdimpl_126 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_126"}
// Dependencies: {}
impl Encode for PathBuf { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_path () . encode (encoder) } }
};
}
