// Generated macro for impl_264 (impl)
macro_rules! Depcrate_serimpl_264 {
() => {
// Module: crate::ser
// Provides: {"impl_264"}
// Dependencies: {}
impl < T > BorshSerialize for Option < T > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { match self { None => 0u8 . serialize (writer) , Some (value) => { 1u8 . serialize (writer) ? ; value . serialize (writer) } } } }
};
}
