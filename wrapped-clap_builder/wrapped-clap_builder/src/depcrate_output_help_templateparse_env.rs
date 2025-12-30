// Generated macro for parse_env (function)
macro_rules! Depcrate_output_help_templateparse_env {
() => {
// Module: crate::output::help_template
// Provides: {"parse_env"}
// Dependencies: {}
# [cfg (feature = "wrap_help")] fn parse_env (var : & str) -> Option < usize > { some ! (some ! (std :: env :: var_os (var)) . to_str ()) . parse :: < usize > () . ok () }
};
}
