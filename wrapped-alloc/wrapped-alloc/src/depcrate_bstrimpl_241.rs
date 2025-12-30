// Generated macro for impl_241 (impl)
macro_rules! Depcrate_bstrimpl_241 {
() => {
// Module: crate::bstr
// Provides: {"impl_241"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] # [cfg (not (no_rc))] impl From < Rc < [u8] > > for Rc < ByteStr > { # [inline] fn from (s : Rc < [u8] >) -> Rc < ByteStr > { unsafe { Rc :: from_raw (Rc :: into_raw (s) as _) } } }
};
}
