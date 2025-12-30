// Generated macro for PointConversionForm (struct)
macro_rules! Depcrate_ecPointConversionForm {
() => {
// Module: crate::ec
// Provides: {"PointConversionForm"}
// Dependencies: {}
# [doc = " Compressed or Uncompressed conversion"] # [doc = ""] # [doc = " Conversion from the binary value of the point on the curve is performed in one of"] # [doc = " compressed, uncompressed, or hybrid conversions.  The default is compressed, except"] # [doc = " for binary curves."] # [doc = ""] # [doc = " Further documentation is available in the [X9.62] standard."] # [doc = ""] # [doc = " [X9.62]: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.202.2977&rep=rep1&type=pdf"] # [derive (Copy , Clone)] pub struct PointConversionForm (ffi :: point_conversion_form_t) ;
};
}
