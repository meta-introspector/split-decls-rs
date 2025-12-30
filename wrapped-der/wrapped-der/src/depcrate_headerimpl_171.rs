// Generated macro for impl_171 (impl)
macro_rules! Depcrate_headerimpl_171 {
() => {
// Module: crate::header
// Provides: {"impl_171"}
// Dependencies: {}
impl DerOrd for Header { fn der_cmp (& self , other : & Self) -> Result < Ordering > { match self . tag . der_cmp (& other . tag) ? { Ordering :: Equal => self . length . der_cmp (& other . length) , ordering => Ok (ordering) , } } }
};
}
