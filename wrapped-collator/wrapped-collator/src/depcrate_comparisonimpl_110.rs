// Generated macro for impl_110 (impl)
macro_rules! Depcrate_comparisonimpl_110 {
() => {
// Module: crate::comparison
// Provides: {"impl_110"}
// Dependencies: {}
impl < const N : usize > CollationKeySink for SmallVec < [u8 ; N] > { type Error = Infallible ; type State = () ; type Output = () ; fn write (& mut self , _ : & mut Self :: State , buf : & [u8]) -> Result < () , Self :: Error > { self . extend_from_slice (buf) ; Ok (()) } fn finish (& mut self , _ : Self :: State) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
};
}
