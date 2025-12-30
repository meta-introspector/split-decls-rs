// Generated macro for impl_870 (impl)
macro_rules! Depcrate_sharedimpl_870 {
() => {
// Module: crate::shared
// Provides: {"impl_870"}
// Dependencies: {}
impl fmt :: Display for ConnectionId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . iter () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
