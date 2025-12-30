// Generated macro for impl_for_nonzero_integer (macro)
macro_rules! Depcrate_serimpl_for_nonzero_integer {
() => {
// Module: crate::ser
// Provides: {"impl_for_nonzero_integer"}
// Dependencies: {}
macro_rules ! impl_for_nonzero_integer { ($ type : ty) => { impl BorshSerialize for $ type { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { BorshSerialize :: serialize (& self . get () , writer) } } } ; }
};
}
