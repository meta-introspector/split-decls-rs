// Generated macro for ToSocketAddrsFuture (enum)
macro_rules! Depcrate_addrToSocketAddrsFuture {
() => {
// Module: crate::addr
// Provides: {"ToSocketAddrsFuture"}
// Dependencies: {}
pub enum ToSocketAddrsFuture < I > { Resolving (future :: Boxed < io :: Result < I > >) , Ready (io :: Result < I >) , Done , }
};
}
