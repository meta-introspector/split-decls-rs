// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < BS : ArraySize , K : BufferKind > Clone for BlockBuffer < BS , K > { # [inline] fn clone (& self) -> Self { unsafe { ptr :: read (self) } } }
};
}
