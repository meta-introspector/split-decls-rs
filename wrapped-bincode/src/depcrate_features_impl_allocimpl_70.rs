// Generated macro for impl_70 (impl)
macro_rules! Depcrate_features_impl_allocimpl_70 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_70"}
// Dependencies: {}
impl < Context > Decode < Context > for Box < str > { fn decode < D : Decoder > (decoder : & mut D) -> Result < Self , DecodeError > { String :: decode (decoder) . map (String :: into_boxed_str) } }
};
}
