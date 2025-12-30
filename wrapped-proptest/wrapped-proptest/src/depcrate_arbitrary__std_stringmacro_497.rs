// Generated macro for macro_497 (macro)
macro_rules! Depcrate_arbitrary__std_stringmacro_497 {
() => {
// Module: crate::arbitrary::_std::string
// Provides: {"macro_497"}
// Dependencies: {}
arbitrary ! (FromUtf8Error , SFnPtrMap < BoxedStrategy < Vec < u8 >>, Self >; static_map (not_utf8_bytes (true) . boxed () , | bs | String :: from_utf8 (bs) . unwrap_err ())) ;
};
}
