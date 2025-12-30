// Generated macro for gen_string (function)
macro_rules! Depcrate_hpack_test_fuzzgen_string {
() => {
// Module: crate::hpack::test::fuzz
// Provides: {"gen_string"}
// Dependencies: {}
fn gen_string (g : & mut StdRng , min : usize , max : usize) -> String { let bytes : Vec < _ > = (min .. max) . map (| _ | { * g . sample (Slice :: new (b"ABCDEFGHIJKLMNOPQRSTUVabcdefghilpqrstuvwxyz----") . unwrap ()) }) . collect () ; String :: from_utf8 (bytes) . unwrap () }
};
}
