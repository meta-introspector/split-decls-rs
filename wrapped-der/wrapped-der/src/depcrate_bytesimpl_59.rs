// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bytesimpl_59 {
() => {
// Module: crate::bytes
// Provides: {"impl_59"}
// Dependencies: {}
impl DerOrd for BytesRef { fn der_cmp (& self , other : & Self) -> Result < Ordering > { Ok (self . as_slice () . cmp (other . as_slice ())) } }
};
}
