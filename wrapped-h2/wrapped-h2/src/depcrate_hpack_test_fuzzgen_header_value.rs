// Generated macro for gen_header_value (function)
macro_rules! Depcrate_hpack_test_fuzzgen_header_value {
() => {
// Module: crate::hpack::test::fuzz
// Provides: {"gen_header_value"}
// Dependencies: {}
fn gen_header_value (g : & mut StdRng) -> HeaderValue { let value = gen_string (g , 0 , 70) ; HeaderValue :: from_bytes (value . as_bytes ()) . unwrap () }
};
}
