// Generated macro for impl_589 (impl)
macro_rules! Depcrate_common_serverimpl_589 {
() => {
// Module: crate::common::server
// Provides: {"impl_589"}
// Dependencies: {}
impl FromStr for Server { type Err = InvalidServer ; fn from_str (src : & str) -> Result < Self , Self :: Err > { HeaderValueString :: from_str (src) . map (Server) . map_err (| _ | InvalidServer { _inner : () }) } }
};
}
