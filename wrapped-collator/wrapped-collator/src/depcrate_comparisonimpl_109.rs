// Generated macro for impl_109 (impl)
macro_rules! Depcrate_comparisonimpl_109 {
() => {
// Module: crate::comparison
// Provides: {"impl_109"}
// Dependencies: {}
impl CollationKeySink for VecDeque < u8 > { type Error = Infallible ; type State = () ; type Output = () ; fn write (& mut self , _ : & mut Self :: State , buf : & [u8]) -> Result < () , Self :: Error > { self . extend (buf . iter ()) ; Ok (()) } fn finish (& mut self , _ : Self :: State) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
};
}
