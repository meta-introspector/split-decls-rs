// Generated macro for macro_290 (macro)
macro_rules! Depcrate_arbitrary__alloc_charmacro_290 {
() => {
// Module: crate::arbitrary::_alloc::char
// Provides: {"macro_290"}
// Dependencies: {}
arbitrary ! (DecodeUtf16Error , SFnPtrMap < Range < u16 >, Self >; static_map (0xD800 .. 0xE000 , | x | decode_utf16 (once (x)) . next () . unwrap () . unwrap_err ())) ;
};
}
