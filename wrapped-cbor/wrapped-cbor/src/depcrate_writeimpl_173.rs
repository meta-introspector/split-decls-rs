// Generated macro for impl_173 (impl)
macro_rules! Depcrate_writeimpl_173 {
() => {
// Module: crate::write
// Provides: {"impl_173"}
// Dependencies: {}
impl < W > Write for & mut W where W : Write , { type Error = W :: Error ; fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { (* self) . write_all (buf) } }
};
}
