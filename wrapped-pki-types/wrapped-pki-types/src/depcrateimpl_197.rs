// Generated macro for impl_197 (impl)
macro_rules! Depcrateimpl_197 {
() => {
// Module: crate
// Provides: {"impl_197"}
// Dependencies: {}
impl CertificateDer < '_ > { # [doc = " Converts this certificate into its owned variant, unfreezing borrowed content (if any)"] # [cfg (feature = "alloc")] pub fn into_owned (self) -> CertificateDer < 'static > { CertificateDer (Der (self . 0 . 0 . into_owned ())) } }
};
}
