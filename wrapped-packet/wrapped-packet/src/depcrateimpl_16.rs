// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl fmt :: Debug for Packet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Packet {{ size: {:?}, addr: {:?} }}" , self . meta . size , self . meta . socket_addr ()) } }
};
}
