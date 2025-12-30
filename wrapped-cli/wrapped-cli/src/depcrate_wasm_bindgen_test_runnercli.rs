// Generated macro for Cli (struct)
macro_rules! Depcrate_wasm_bindgen_test_runnerCli {
() => {
// Module: crate::wasm_bindgen_test_runner
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser)] # [command (name = "wasm-bindgen-test-runner" , version , about , long_about = None)] struct Cli { # [arg (index = 1 , help = "The file to test. `cargo test` passes this argument for you.")] file : PathBuf , # [arg (long , conflicts_with = "ignored" , help = "Run ignored tests")] include_ignored : bool , # [arg (long , conflicts_with = "include_ignored" , help = "Run ignored tests")] ignored : bool , # [arg (long , help = "Exactly match filters rather than by substring")] exact : bool , # [arg (long , value_name = "FILTER" , help = "Skip tests whose names contain FILTER (this flag can be used multiple times)")] skip : Vec < String > , # [arg (long , help = "List all tests and benchmarks")] list : bool , # [arg (long , help = "don't capture `console.*()` of each task, allow printing directly")] nocapture : bool , # [arg (long , value_enum , value_name = "terse" , help = "Configure formatting of output")] format : Option < FormatSetting > , # [arg (index = 2 , value_name = "FILTER" , help = "The FILTER string is tested against the name of all tests, and only those tests \
                whose names contain the filter are run.")] filter : Option < String > , }
};
}
