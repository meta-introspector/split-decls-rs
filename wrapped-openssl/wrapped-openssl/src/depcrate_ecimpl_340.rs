// Generated macro for impl_340 (impl)
macro_rules! Depcrate_ecimpl_340 {
() => {
// Module: crate::ec
// Provides: {"impl_340"}
// Dependencies: {}
impl PointConversionForm { # [doc = " Compressed conversion from point value."] pub const COMPRESSED : PointConversionForm = PointConversionForm (ffi :: point_conversion_form_t :: POINT_CONVERSION_COMPRESSED) ; # [doc = " Uncompressed conversion from point value."] pub const UNCOMPRESSED : PointConversionForm = PointConversionForm (ffi :: point_conversion_form_t :: POINT_CONVERSION_UNCOMPRESSED) ; # [doc = " Performs both compressed and uncompressed conversions."] pub const HYBRID : PointConversionForm = PointConversionForm (ffi :: point_conversion_form_t :: POINT_CONVERSION_HYBRID) ; }
};
}
