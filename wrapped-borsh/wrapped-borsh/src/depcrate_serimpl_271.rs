// Generated macro for impl_271 (impl)
macro_rules! Depcrate_serimpl_271 {
() => {
// Module: crate::ser
// Provides: {"impl_271"}
// Dependencies: {}
impl < T : BorshSerialize + ? Sized > BorshSerialize for & T { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { (* self) . serialize (writer) } }
};
}
