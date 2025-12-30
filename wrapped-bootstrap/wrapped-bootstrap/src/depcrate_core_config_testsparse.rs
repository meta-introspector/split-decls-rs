// Generated macro for parse (function)
macro_rules! Depcrate_core_config_testsparse {
() => {
// Module: crate::core::config::tests
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (config : & str) -> Config { Config :: parse_inner (Flags :: parse (& ["check" . to_string () , "--config=/does/not/exist" . to_string ()]) , | & _ | toml :: from_str (& config) ,) }
};
}
