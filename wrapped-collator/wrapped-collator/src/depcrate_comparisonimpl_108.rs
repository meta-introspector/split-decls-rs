// Generated macro for impl_108 (impl)
macro_rules! Depcrate_comparisonimpl_108 {
() => {
// Module: crate::comparison
// Provides: {"impl_108"}
// Dependencies: {}
impl CollationKeySink for Vec < u8 > { type Error = Infallible ; type State = () ; type Output = () ; fn write (& mut self , _ : & mut Self :: State , buf : & [u8]) -> Result < () , Self :: Error > { self . extend_from_slice (buf) ; Ok (()) } fn finish (& mut self , _ : Self :: State) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
};
}
