// Generated macro for impl_for_integer (macro)
macro_rules! Depcrate_serimpl_for_integer {
() => {
// Module: crate::ser
// Provides: {"impl_for_integer"}
// Dependencies: {}
macro_rules ! impl_for_integer { ($ type : ident) => { impl BorshSerialize for $ type { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { let bytes = self . to_le_bytes () ; writer . write_all (& bytes) } } } ; }
};
}
