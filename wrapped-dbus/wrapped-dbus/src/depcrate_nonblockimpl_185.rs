// Generated macro for impl_185 (impl)
macro_rules! Depcrate_nonblockimpl_185 {
() => {
// Module: crate::nonblock
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'a , C > Proxy < 'a , C > { # [doc = " Creates a new proxy struct."] pub fn new < D : Into < BusName < 'a > > , P : Into < Path < 'a > > > (dest : D , path : P , timeout : Duration , connection : C) -> Self { Proxy { destination : dest . into () , path : path . into () , timeout , connection } } }
};
}
