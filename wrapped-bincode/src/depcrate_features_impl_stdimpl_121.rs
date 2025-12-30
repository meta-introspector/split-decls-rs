// Generated macro for impl_121 (impl)
macro_rules! Depcrate_features_impl_stdimpl_121 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_121"}
// Dependencies: {}
impl Encode for SystemTime { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let duration = self . duration_since (SystemTime :: UNIX_EPOCH) . map_err (| e | { EncodeError :: InvalidSystemTime { inner : e , time : std :: boxed :: Box :: new (* self) , } }) ? ; duration . encode (encoder) } }
};
}
