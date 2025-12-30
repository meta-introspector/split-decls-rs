// Generated macro for format_env_variables (function)
macro_rules! Depcrate_utilsformat_env_variables {
() => {
// Module: crate::utils
// Provides: {"format_env_variables"}
// Dependencies: {}
pub fn format_env_variables () -> String { let vars = std :: env :: vars () . map (| (key , value) | format ! ("{key}={value}")) . collect :: < Vec < _ > > () ; vars . join ("\n") }
};
}
