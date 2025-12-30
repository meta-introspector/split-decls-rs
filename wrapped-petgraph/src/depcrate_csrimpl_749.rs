// Generated macro for impl_749 (impl)
macro_rules! Depcrate_csrimpl_749 {
() => {
// Module: crate::csr
// Provides: {"impl_749"}
// Dependencies: {}
impl < 'a , Ty , E , Ix > EdgeReference < 'a , E , Ty , Ix > where Ty : EdgeType , { # [doc = " Access the edge’s weight."] # [doc = ""] # [doc = " **NOTE** that this method offers a longer lifetime"] # [doc = " than the trait (unfortunately they don't match yet)."] pub fn weight (& self) -> & 'a E { self . weight } }
};
}
