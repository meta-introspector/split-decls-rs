// Generated macro for impl_290 (impl)
macro_rules! Depcrate_serimpl_290 {
() => {
// Module: crate::ser
// Provides: {"impl_290"}
// Dependencies: {}
impl < T : BorshSerialize + ? Sized > BorshSerialize for Box < T > { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_ref () . serialize (writer) } }
};
}
