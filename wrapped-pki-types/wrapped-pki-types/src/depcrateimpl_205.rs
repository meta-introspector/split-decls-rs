// Generated macro for impl_205 (impl)
macro_rules! Depcrateimpl_205 {
() => {
// Module: crate
// Provides: {"impl_205"}
// Dependencies: {}
impl SubjectPublicKeyInfoDer < '_ > { # [doc = " Converts this SubjectPublicKeyInfo into its owned variant, unfreezing borrowed content (if any)"] # [cfg (feature = "alloc")] pub fn into_owned (self) -> SubjectPublicKeyInfoDer < 'static > { SubjectPublicKeyInfoDer (Der (self . 0 . 0 . into_owned ())) } }
};
}
