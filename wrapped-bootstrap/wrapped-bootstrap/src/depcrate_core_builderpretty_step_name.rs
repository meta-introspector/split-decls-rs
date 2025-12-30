// Generated macro for pretty_step_name (function)
macro_rules! Depcrate_core_builderpretty_step_name {
() => {
// Module: crate::core::builder
// Provides: {"pretty_step_name"}
// Dependencies: {}
# [doc = " Return qualified step name, e.g. `compile::Rustc`."] pub fn pretty_step_name < S : Step > () -> String { let path = type_name :: < S > () . rsplit ("::") . take (2) . collect :: < Vec < _ > > () ; path . into_iter () . rev () . collect :: < Vec < _ > > () . join ("::") }
};
}
