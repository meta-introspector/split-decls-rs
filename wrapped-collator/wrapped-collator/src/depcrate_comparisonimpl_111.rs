// Generated macro for impl_111 (impl)
macro_rules! Depcrate_comparisonimpl_111 {
() => {
// Module: crate::comparison
// Provides: {"impl_111"}
// Dependencies: {}
impl CollationKeySink for [u8] { type Error = TooSmall ; type State = usize ; type Output = usize ; fn write (& mut self , offset : & mut Self :: State , buf : & [u8]) -> Result < () , Self :: Error > { if * offset + buf . len () <= self . len () { # [expect (clippy :: indexing_slicing)] self [* offset .. * offset + buf . len ()] . copy_from_slice (buf) ; } * offset += buf . len () ; Ok (()) } fn finish (& mut self , offset : Self :: State) -> Result < Self :: Output , Self :: Error > { if offset <= self . len () { Ok (offset) } else { Err (TooSmall :: new (offset)) } } }
};
}
