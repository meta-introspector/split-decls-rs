// Generated macro for impl_38 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_38 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_38"}
// Dependencies: {}
impl ResponseFuture { fn new < F > (value : F) -> Self where F : Future < Output = Result < Response < hyper :: body :: Incoming > , Error > > + Send + 'static , { Self { inner : SyncWrapper :: new (Box :: pin (value)) , } } fn error_version (ver : Version) -> Self { warn ! ("Request has unsupported version \"{:?}\"" , ver) ; ResponseFuture :: new (Box :: pin (future :: err (e ! (UserUnsupportedVersion)))) } }
};
}
