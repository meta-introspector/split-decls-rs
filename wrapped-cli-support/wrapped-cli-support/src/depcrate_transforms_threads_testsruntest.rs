// Generated macro for runtest (function)
macro_rules! Depcrate_transforms_threads_testsruntest {
() => {
// Module: crate::transforms::threads::tests
// Provides: {"runtest"}
// Dependencies: {}
fn runtest (test : & Test) -> Result < String > { let wasm = wat :: parse_file (& test . file) ? ; let mut module = ModuleConfig :: new () . generate_producers_section (false) . parse (& wasm) ? ; super :: run (& mut module) ? ; walrus :: passes :: gc :: run (& mut module) ; unstart_start_function (& mut module) ; let features = wasmparser :: WasmFeatures :: default () | wasmparser :: WasmFeatures :: THREADS ; wasmparser :: Validator :: new_with_features (features) . validate_all (& module . emit_wasm ()) ? ; let printed = wasmprinter :: print_bytes (module . emit_wasm ()) ? ; Ok (printed) }
};
}
