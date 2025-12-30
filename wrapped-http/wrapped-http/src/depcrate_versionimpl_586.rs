// Generated macro for impl_586 (impl)
macro_rules! Depcrate_versionimpl_586 {
() => {
// Module: crate::version
// Provides: {"impl_586"}
// Dependencies: {}
impl fmt :: Debug for Version { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: Http :: * ; f . write_str (match self . 0 { Http09 => "HTTP/0.9" , Http10 => "HTTP/1.0" , Http11 => "HTTP/1.1" , H2 => "HTTP/2.0" , H3 => "HTTP/3.0" , __NonExhaustive => unreachable ! () , }) } }
};
}
