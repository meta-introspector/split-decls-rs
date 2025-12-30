// Generated macro for impl_273 (impl)
macro_rules! Depcrate_serimpl_273 {
() => {
// Module: crate::ser
// Provides: {"impl_273"}
// Dependencies: {}
impl < T > BorshSerialize for Vec < T > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; self . as_slice () . serialize (writer) } }
};
}
