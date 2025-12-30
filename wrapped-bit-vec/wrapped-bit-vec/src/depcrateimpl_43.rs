// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < B : BitBlock > Clone for BitVec < B > { # [inline] fn clone (& self) -> Self { self . ensure_invariant () ; BitVec { storage : self . storage . clone () , nbits : self . nbits , } } # [inline] fn clone_from (& mut self , source : & Self) { debug_assert ! (source . is_last_block_fixed ()) ; self . nbits = source . nbits ; self . storage . clone_from (& source . storage) ; } }
};
}
