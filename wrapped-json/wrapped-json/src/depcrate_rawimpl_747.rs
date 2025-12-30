// Generated macro for impl_747 (impl)
macro_rules! Depcrate_rawimpl_747 {
() => {
// Module: crate::raw
// Provides: {"impl_747"}
// Dependencies: {}
impl Debug for RawValue { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_tuple ("RawValue") . field (& format_args ! ("{}" , & self . json)) . finish () } }
};
}
