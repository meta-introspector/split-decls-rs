// Generated macro for impl_263 (impl)
macro_rules! Depcrate_serimpl_263 {
() => {
// Module: crate::ser
// Provides: {"impl_263"}
// Dependencies: {}
impl BorshSerialize for bool { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { (u8 :: from (* self)) . serialize (writer) } }
};
}
