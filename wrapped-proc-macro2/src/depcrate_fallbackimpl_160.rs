// Generated macro for impl_160 (impl)
macro_rules! Depcrate_fallbackimpl_160 {
() => {
// Module: crate::fallback
// Provides: {"impl_160"}
// Dependencies: {}
impl < T > PartialEq < T > for Ident where T : ? Sized + AsRef < str > , { fn eq (& self , other : & T) -> bool { let other = other . as_ref () ; if self . raw { other . starts_with ("r#") && * self . sym == other [2 ..] } else { * self . sym == * other } } }
};
}
