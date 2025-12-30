// Generated macro for impl_989 (impl)
macro_rules! Depcrateimpl_989 {
() => {
// Module: crate
// Provides: {"impl_989"}
// Dependencies: {}
impl fmt :: Display for StreamId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let initiator = match self . initiator () { Side :: Client => "client" , Side :: Server => "server" , } ; let dir = match self . dir () { Dir :: Uni => "uni" , Dir :: Bi => "bi" , } ; write ! (f , "{} {}directional stream {}" , initiator , dir , self . index ()) } }
};
}
