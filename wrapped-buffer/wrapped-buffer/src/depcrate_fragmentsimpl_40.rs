// Generated macro for impl_40 (impl)
macro_rules! Depcrate_fragmentsimpl_40 {
() => {
// Module: crate::fragments
// Provides: {"impl_40"}
// Dependencies: {}
impl Fragment for str { # [cfg (feature = "alloc")] # [inline (always)] fn extend (buf : & mut Cow < Self > , fragment : & Self) { buf . to_mut () . push_str (fragment) ; } fn can_replace (& self) -> bool { self . len () == 0 } }
};
}
