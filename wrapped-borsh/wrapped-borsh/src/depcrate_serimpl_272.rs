// Generated macro for impl_272 (impl)
macro_rules! Depcrate_serimpl_272 {
() => {
// Module: crate::ser
// Provides: {"impl_272"}
// Dependencies: {}
impl < T > BorshSerialize for Cow < '_ , T > where T : BorshSerialize + ToOwned + ? Sized , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_ref () . serialize (writer) } }
};
}
