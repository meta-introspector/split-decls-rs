// Generated macro for impl_982 (impl)
macro_rules! Depcrate_util_colorimpl_982 {
() => {
// Module: crate::util::color
// Provides: {"impl_982"}
// Dependencies: {}
impl ColorChoice { # [doc = " Report all `possible_values`"] pub fn possible_values () -> impl Iterator < Item = PossibleValue > { Self :: value_variants () . iter () . filter_map (ValueEnum :: to_possible_value) } }
};
}
