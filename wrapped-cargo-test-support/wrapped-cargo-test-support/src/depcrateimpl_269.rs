// Generated macro for impl_269 (impl)
macro_rules! Depcrateimpl_269 {
() => {
// Module: crate
// Provides: {"impl_269"}
// Dependencies: {}
impl ChannelChangerCommandExt for snapbox :: cmd :: Command { fn masquerade_as_nightly_cargo (self , _reasons : & [& str]) -> Self { self . env ("__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS" , "nightly") } }
};
}
