// Generated macro for to_x962_uncompressed (function)
macro_rules! Depcrate_ecto_x962_uncompressed {
() => {
// Module: crate::ec
// Provides: {"to_x962_uncompressed"}
// Dependencies: {}
# [doc = " Serialize a finite point to uncompressed X9.62 format."] # [doc = ""] # [doc = " Callers must ensure that the arguments are valid, that the point has the"] # [doc = " specified group, and that the point is finite."] unsafe fn to_x962_uncompressed (group : * const bssl_sys :: EC_GROUP , point : * const bssl_sys :: EC_POINT ,) -> Buffer { cbb_to_buffer (65 , | cbb | unsafe { let result = bssl_sys :: EC_POINT_point2cbb (cbb , group , point , bssl_sys :: point_conversion_form_t :: POINT_CONVERSION_UNCOMPRESSED , null_mut () ,) ; assert_eq ! (result , 1) ; }) }
};
}
