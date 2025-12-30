// Generated macro for impl_902 (impl)
macro_rules! Depcrate_packetimpl_902 {
() => {
// Module: crate::packet
// Provides: {"impl_902"}
// Dependencies: {}
impl std :: fmt :: Debug for ConnectionId < '_ > { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { for c in self . as_ref () { write ! (f , "{c:02x}") ? ; } Ok (()) } }
};
}
