// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_runtime_nsproxyimpl_1213 {
() => {
// Module: crate::runtime::nsproxy
// Provides: {"impl_1213"}
// Dependencies: {}
impl fmt :: Debug for NSProxy { # [inline] # [doc (alias = "description")] # [doc (alias = "debugDescription")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let obj : & ProtocolObject < dyn NSObjectProtocol > = ProtocolObject :: from_ref (self) ; obj . fmt (f) } }
};
}
