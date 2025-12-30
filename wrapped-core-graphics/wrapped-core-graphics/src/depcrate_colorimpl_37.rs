// Generated macro for impl_37 (impl)
macro_rules! Depcrate_colorimpl_37 {
() => {
// Module: crate::color
// Provides: {"impl_37"}
// Dependencies: {}
impl CGColor { pub fn rgb (red : CGFloat , green : CGFloat , blue : CGFloat , alpha : CGFloat) -> Self { unsafe { let ptr = CGColorCreateGenericRGB (red , green , blue , alpha) ; CGColor :: wrap_under_create_rule (ptr) } } # [cfg (feature = "catalina")] pub fn srgb (red : CGFloat , green : CGFloat , blue : CGFloat , alpha : CGFloat) -> Self { unsafe { let ptr = CGColorCreateSRGB (red , green , blue , alpha) ; CGColor :: wrap_under_create_rule (ptr) } } }
};
}
