// Generated macro for impl_242 (impl)
macro_rules! Depcrate_bstrimpl_242 {
() => {
// Module: crate::bstr
// Provides: {"impl_242"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] # [cfg (not (no_rc))] impl From < Rc < ByteStr > > for Rc < [u8] > { # [inline] fn from (s : Rc < ByteStr >) -> Rc < [u8] > { unsafe { Rc :: from_raw (Rc :: into_raw (s) as _) } } }
};
}
