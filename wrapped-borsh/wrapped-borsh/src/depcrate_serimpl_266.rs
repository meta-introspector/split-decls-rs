// Generated macro for impl_266 (impl)
macro_rules! Depcrate_serimpl_266 {
() => {
// Module: crate::ser
// Provides: {"impl_266"}
// Dependencies: {}
impl BorshSerialize for str { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_bytes () . serialize (writer) } }
};
}
