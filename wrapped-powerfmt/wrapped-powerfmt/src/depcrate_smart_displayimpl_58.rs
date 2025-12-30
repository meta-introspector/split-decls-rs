// Generated macro for impl_58 (impl)
macro_rules! Depcrate_smart_displayimpl_58 {
() => {
// Module: crate::smart_display
// Provides: {"impl_58"}
// Dependencies: {}
impl Metadata < '_ , Infallible > { # [doc = " Obtain the width of the value before padding, given the formatter options."] pub fn unpadded_width_of < T > (value : T , f : FormatterOptions) -> usize where T : SmartDisplay , { value . metadata (f) . unpadded_width } # [doc = " Obtain the width of the value after padding, given the formatter options."] pub fn padded_width_of < T > (value : T , f : FormatterOptions) -> usize where T : SmartDisplay , { value . metadata (f) . padded_width (f) } }
};
}
