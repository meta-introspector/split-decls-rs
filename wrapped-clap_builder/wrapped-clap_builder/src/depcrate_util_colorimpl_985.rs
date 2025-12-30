// Generated macro for impl_985 (impl)
macro_rules! Depcrate_util_colorimpl_985 {
() => {
// Module: crate::util::color
// Provides: {"impl_985"}
// Dependencies: {}
impl ValueEnum for ColorChoice { fn value_variants < 'a > () -> & 'a [Self] { & [Self :: Auto , Self :: Always , Self :: Never] } fn to_possible_value (& self) -> Option < PossibleValue > { Some (match self { Self :: Auto => PossibleValue :: new ("auto") , Self :: Always => PossibleValue :: new ("always") , Self :: Never => PossibleValue :: new ("never") , }) } }
};
}
