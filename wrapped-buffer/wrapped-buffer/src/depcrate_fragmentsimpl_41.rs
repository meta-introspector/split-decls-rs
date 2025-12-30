// Generated macro for impl_41 (impl)
macro_rules! Depcrate_fragmentsimpl_41 {
() => {
// Module: crate::fragments
// Provides: {"impl_41"}
// Dependencies: {}
impl Fragment for [u8] { # [cfg (feature = "alloc")] # [inline (always)] fn extend (buf : & mut Cow < Self > , fragment : & Self) { buf . to_mut () . extend (fragment) ; } fn can_replace (& self) -> bool { self . len () == 0 } }
};
}
