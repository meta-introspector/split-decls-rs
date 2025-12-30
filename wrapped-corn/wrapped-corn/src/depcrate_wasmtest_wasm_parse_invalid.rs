// Generated macro for test_wasm_parse_invalid (function)
macro_rules! Depcrate_wasmtest_wasm_parse_invalid {
() => {
// Module: crate::wasm
// Provides: {"test_wasm_parse_invalid"}
// Dependencies: {}
# [cfg (test)] # [wasm_bindgen_test] fn test_wasm_parse_invalid () { let res = parse ("{foo = \"$bar\"}") ; assert ! (res . is_err ()) }
};
}
