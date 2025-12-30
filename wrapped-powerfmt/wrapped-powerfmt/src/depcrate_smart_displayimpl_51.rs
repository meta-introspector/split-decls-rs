// Generated macro for impl_51 (impl)
macro_rules! Depcrate_smart_displayimpl_51 {
() => {
// Module: crate::smart_display
// Provides: {"impl_51"}
// Dependencies: {}
impl From < & Formatter < '_ > > for FormatterOptions { fn from (value : & Formatter < '_ >) -> Self { * Self :: default () . with_fill (value . fill ()) . with_sign_plus (value . sign_plus ()) . with_sign_minus (value . sign_minus ()) . with_align (value . align ()) . with_width (value . width ()) . with_precision (value . precision ()) . with_alternate (value . alternate ()) . with_sign_aware_zero_pad (value . sign_aware_zero_pad ()) } }
};
}
