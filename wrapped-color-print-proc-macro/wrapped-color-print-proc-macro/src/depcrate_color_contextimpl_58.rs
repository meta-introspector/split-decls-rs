// Generated macro for impl_58 (impl)
macro_rules! Depcrate_color_contextimpl_58 {
() => {
// Module: crate::color_context
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "terminfo")] impl < T > Action < T > { pub fn actual_value (& self) -> Option < & T > { match self { Action :: Keep (val) | Action :: Change (val) => Some (val) , Action :: None => None , } } }
};
}
