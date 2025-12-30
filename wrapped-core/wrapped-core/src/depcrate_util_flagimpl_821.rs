// Generated macro for impl_821 (impl)
macro_rules! Depcrate_util_flagimpl_821 {
() => {
// Module: crate::util::flag
// Provides: {"impl_821"}
// Dependencies: {}
impl FromMeta for Flag { fn from_none () -> Option < Self > { Some (Flag (None)) } fn from_meta (mi : & syn :: Meta) -> Result < Self > { if let Meta :: Path (p) = mi { Ok (Flag (Some (p . span ()))) } else { Err (< () > :: from_meta (mi) . unwrap_err ()) } } }
};
}
