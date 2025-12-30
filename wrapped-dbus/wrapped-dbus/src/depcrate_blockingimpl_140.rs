// Generated macro for impl_140 (impl)
macro_rules! Depcrate_blockingimpl_140 {
() => {
// Module: crate::blocking
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , C > Proxy < 'a , C > { # [doc = " Creates a new proxy struct."] pub fn new < D : Into < BusName < 'a > > , P : Into < Path < 'a > > > (dest : D , path : P , timeout : Duration , connection : C) -> Self { Proxy { destination : dest . into () , path : path . into () , timeout , connection } } }
};
}
