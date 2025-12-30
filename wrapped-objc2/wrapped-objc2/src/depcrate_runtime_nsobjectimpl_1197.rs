// Generated macro for impl_1197 (impl)
macro_rules! Depcrate_runtime_nsobjectimpl_1197 {
() => {
// Module: crate::runtime::nsobject
// Provides: {"impl_1197"}
// Dependencies: {}
impl fmt :: Debug for NSObject { # [inline] # [doc (alias = "description")] # [doc (alias = "debugDescription")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let obj : & ProtocolObject < dyn NSObjectProtocol > = ProtocolObject :: from_ref (self) ; obj . fmt (f) } }
};
}
