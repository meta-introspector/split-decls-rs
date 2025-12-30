// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < B : BitBlock > Clone for BitSet < B > { fn clone (& self) -> Self { BitSet { bit_vec : self . bit_vec . clone () , } } fn clone_from (& mut self , other : & Self) { self . bit_vec . clone_from (& other . bit_vec) ; } }
};
}
