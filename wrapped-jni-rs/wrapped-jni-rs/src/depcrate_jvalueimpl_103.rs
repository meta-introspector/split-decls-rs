// Generated macro for impl_103 (impl)
macro_rules! Depcrate_jvalueimpl_103 {
() => {
// Module: crate::jvalue
// Provides: {"impl_103"}
// Dependencies: {}
impl TryFrom < JValueOwned < '_ > > for jboolean { type Error = Error ; fn try_from (value : JValueOwned) -> Result < Self > { value . borrow () . try_into () } }
};
}
