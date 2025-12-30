// Generated macro for escape_step_name (function)
macro_rules! Depcrate_metricsescape_step_name {
() => {
// Module: crate::metrics
// Provides: {"escape_step_name"}
// Dependencies: {}
# [doc = " Bootstrap steps can be generic and thus contain angle brackets (<...>)."] # [doc = " However, Markdown interprets these as HTML, so we need to escap ethem."] pub fn escape_step_name (step : & BuildStep) -> String { step . r#type . replace ('<' , "&lt;") . replace ('>' , "&gt;") }
};
}
