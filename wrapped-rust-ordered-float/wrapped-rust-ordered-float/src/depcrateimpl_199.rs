// Generated macro for impl_199 (impl)
macro_rules! Depcrateimpl_199 {
() => {
// Module: crate
// Provides: {"impl_199"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : fmt :: Debug + Error + 'static > Error for ParseNotNanError < E > { fn description (& self) -> & str { "Error parsing a not-NaN floating point value" } fn source (& self) -> Option < & (dyn Error + 'static) > { match self { ParseNotNanError :: ParseFloatError (e) => Some (e) , ParseNotNanError :: IsNaN => None , } } }
};
}
