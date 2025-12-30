// Generated macro for impl_808 (impl)
macro_rules! Depcrate_rawimpl_808 {
() => {
// Module: crate::raw
// Provides: {"impl_808"}
// Dependencies: {}
impl Debug for RawValue { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_tuple ("RawValue") . field (& format_args ! ("{}" , & self . json)) . finish () } }
};
}
