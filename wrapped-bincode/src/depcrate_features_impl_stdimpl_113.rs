// Generated macro for impl_113 (impl)
macro_rules! Depcrate_features_impl_stdimpl_113 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_113"}
// Dependencies: {}
impl < Context > Decode < Context > for CString { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = std :: vec :: Vec :: decode (decoder) ? ; CString :: new (vec) . map_err (| inner | DecodeError :: CStringNulError { position : inner . nul_position () , }) } }
};
}
