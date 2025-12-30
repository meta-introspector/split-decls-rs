// Generated macro for impl_401 (impl)
macro_rules! Depcrate_builder_value_parserimpl_401 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_401"}
// Dependencies: {}
impl FalseyValueParser { # [doc = " Parse false-like string values, everything else is `true`"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { crate :: util :: TRUE_LITERALS . iter () . chain (crate :: util :: FALSE_LITERALS . iter ()) . copied () . map (| l | crate :: builder :: PossibleValue :: new (l) . hide (l != "true" && l != "false")) } }
};
}
