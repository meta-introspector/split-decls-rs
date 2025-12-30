// Generated macro for impl_809 (impl)
macro_rules! Depcrate_clientimpl_809 {
() => {
// Module: crate::client
// Provides: {"impl_809"}
// Dependencies: {}
impl < T , B > fmt :: Debug for Connection < T , B > where T : AsyncRead + AsyncWrite , T : fmt :: Debug , B : fmt :: Debug + Buf , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . inner , fmt) } }
};
}
