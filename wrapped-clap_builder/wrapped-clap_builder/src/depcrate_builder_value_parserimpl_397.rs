// Generated macro for impl_397 (impl)
macro_rules! Depcrate_builder_value_parserimpl_397 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_397"}
// Dependencies: {}
impl BoolValueParser { # [doc = " Implementation for [`ValueParser::bool`]"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { ["true" , "false"] . iter () . copied () . map (crate :: builder :: PossibleValue :: new) } }
};
}
