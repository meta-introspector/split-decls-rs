// Generated macro for impl_529 (impl)
macro_rules! Depcrate_common_refererimpl_529 {
() => {
// Module: crate::common::referer
// Provides: {"impl_529"}
// Dependencies: {}
impl FromStr for Referer { type Err = InvalidReferer ; fn from_str (src : & str) -> Result < Self , Self :: Err > { HeaderValueString :: from_str (src) . map (Referer) . map_err (| _ | InvalidReferer { _inner : () }) } }
};
}
