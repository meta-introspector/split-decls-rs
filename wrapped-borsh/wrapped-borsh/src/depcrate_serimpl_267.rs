// Generated macro for impl_267 (impl)
macro_rules! Depcrate_serimpl_267 {
() => {
// Module: crate::ser
// Provides: {"impl_267"}
// Dependencies: {}
impl BorshSerialize for String { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_bytes () . serialize (writer) } }
};
}
