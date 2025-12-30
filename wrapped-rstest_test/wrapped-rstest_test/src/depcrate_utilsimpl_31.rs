// Generated macro for impl_31 (impl)
macro_rules! Depcrate_utilsimpl_31 {
() => {
// Module: crate::utils
// Provides: {"impl_31"}
// Dependencies: {}
impl < B : AsRef < [u8] > > Stringable for B { fn str (& self) -> Cow < '_ , str > { String :: from_utf8_lossy (self . as_ref ()) } }
};
}
