// Generated macro for impl_68 (impl)
macro_rules! Depcrate_features_impl_allocimpl_68 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_68"}
// Dependencies: {}
impl < Context > Decode < Context > for String { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let bytes = Vec :: < u8 > :: decode (decoder) ? ; String :: from_utf8 (bytes) . map_err (| e | DecodeError :: Utf8 { inner : e . utf8_error () , }) } }
};
}
