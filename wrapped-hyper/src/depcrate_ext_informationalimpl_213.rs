// Generated macro for impl_213 (impl)
macro_rules! Depcrate_ext_informationalimpl_213 {
() => {
// Module: crate::ext::informational
// Provides: {"impl_213"}
// Dependencies: {}
impl < F > OnInformationalCallback for OnInformationalClosure < F > where F : Fn (Response < '_ >) + Send + Sync + 'static , { fn on_informational (& self , res : http :: Response < () >) { let res = Response (& res) ; (self . 0) (res) ; } }
};
}
