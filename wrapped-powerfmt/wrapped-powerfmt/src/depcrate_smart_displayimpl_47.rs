// Generated macro for impl_47 (impl)
macro_rules! Depcrate_smart_displayimpl_47 {
() => {
// Module: crate::smart_display
// Provides: {"impl_47"}
// Dependencies: {}
impl Debug for FormatterOptions { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { f . debug_struct ("FormatterOptions") . field ("fill" , & self . fill) . field ("align" , & self . align ()) . field ("width" , & self . width ()) . field ("precision" , & self . precision ()) . field ("sign_plus" , & self . sign_plus ()) . field ("sign_minus" , & self . sign_minus ()) . field ("alternate" , & self . alternate ()) . field ("sign_aware_zero_pad" , & self . sign_aware_zero_pad ()) . finish () } }
};
}
