// Generated macro for impl_405 (impl)
macro_rules! Depcrate_builder_value_parserimpl_405 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_405"}
// Dependencies: {}
impl BoolishValueParser { # [doc = " Parse bool-like string values"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { crate :: util :: TRUE_LITERALS . iter () . chain (crate :: util :: FALSE_LITERALS . iter ()) . copied () . map (| l | crate :: builder :: PossibleValue :: new (l) . hide (l != "true" && l != "false")) } }
};
}
