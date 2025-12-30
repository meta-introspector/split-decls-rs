// Generated macro for impl_38 (impl)
macro_rules! Depcrate_wasm_bindgen_test_runnerimpl_38 {
() => {
// Module: crate::wasm_bindgen_test_runner
// Provides: {"impl_38"}
// Dependencies: {}
impl Cli { fn into_args (self , tests : & Tests) -> String { let include_ignored = self . include_ignored ; let filtered = tests . filtered ; format ! (r#"
            // Forward runtime arguments.
            cx.include_ignored({include_ignored:?});
            cx.filtered_count({filtered});
        "#) } }
};
}
