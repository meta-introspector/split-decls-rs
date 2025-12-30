// Generated macro for impl_635 (impl)
macro_rules! Depcrate_common_user_agentimpl_635 {
() => {
// Module: crate::common::user_agent
// Provides: {"impl_635"}
// Dependencies: {}
impl FromStr for UserAgent { type Err = InvalidUserAgent ; fn from_str (src : & str) -> Result < Self , Self :: Err > { HeaderValueString :: from_str (src) . map (UserAgent) . map_err (| _ | InvalidUserAgent { _inner : () }) } }
};
}
