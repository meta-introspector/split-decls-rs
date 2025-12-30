// Generated macro for impl_847 (impl)
macro_rules! Depcrate_serverimpl_847 {
() => {
// Module: crate::server
// Provides: {"impl_847"}
// Dependencies: {}
impl < B : Buf + fmt :: Debug > fmt :: Debug for SendPushedResponse < B > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "SendPushedResponse {{ {:?} }}" , self . inner) } }
};
}
