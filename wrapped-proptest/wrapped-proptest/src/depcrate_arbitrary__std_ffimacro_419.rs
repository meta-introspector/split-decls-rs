// Generated macro for macro_419 (macro)
macro_rules! Depcrate_arbitrary__std_ffimacro_419 {
() => {
// Module: crate::arbitrary::_std::ffi
// Provides: {"macro_419"}
// Dependencies: {}
arbitrary ! (IntoStringError , SFnPtrMap < BoxedStrategy < Vec < u8 >>, Self >; static_map (not_utf8_bytes (false) . boxed () , | bytes | CString :: new (bytes) . unwrap () . into_string () . unwrap_err ())) ;
};
}
