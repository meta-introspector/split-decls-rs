// Generated macro for impl_20 (impl)
macro_rules! Depcrate_dev_macimpl_20 {
() => {
// Module: crate::dev::mac
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , T : OutputSizeUser > From < & 'a Output < T > > for CtOutput < T > { # [inline (always)] fn from (bytes : & 'a Output < T >) -> Self { bytes . clone () . into () } }
};
}
