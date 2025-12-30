// Generated macro for impl_276 (impl)
macro_rules! Depcrate_value_rawimpl_276 {
() => {
// Module: crate::value::raw
// Provides: {"impl_276"}
// Dependencies: {}
impl fmt :: Debug for RawValue { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("RawValue") . field (& format_args ! ("{}" , & self . ron)) . finish () } }
};
}
