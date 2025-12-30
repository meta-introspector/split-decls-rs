// Generated macro for impl_869 (impl)
macro_rules! Depcrate_serverimpl_869 {
() => {
// Module: crate::server
// Provides: {"impl_869"}
// Dependencies: {}
impl < T , B > fmt :: Debug for Handshaking < T , B > where B : Buf , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match * self { Handshaking :: Flushing (_) => f . write_str ("Flushing(_)") , Handshaking :: ReadingPreface (_) => f . write_str ("ReadingPreface(_)") , Handshaking :: Done => f . write_str ("Done") , } } }
};
}
