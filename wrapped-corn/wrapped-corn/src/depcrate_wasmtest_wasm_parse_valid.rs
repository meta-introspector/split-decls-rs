// Generated macro for test_wasm_parse_valid (function)
macro_rules! Depcrate_wasmtest_wasm_parse_valid {
() => {
// Module: crate::wasm
// Provides: {"test_wasm_parse_valid"}
// Dependencies: {}
# [cfg (test)] # [wasm_bindgen_test] fn test_wasm_parse_valid () { let res = parse ("{foo = \"bar\"}") ; assert ! (res . is_ok ()) }
};
}
