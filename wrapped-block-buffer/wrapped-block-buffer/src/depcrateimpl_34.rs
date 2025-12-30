// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < BS : ArraySize , K : BufferKind > BlockBuffer < BS , K > { # [doc = " This associated constant is used to assert block size correctness at compile time."] const BLOCK_SIZE_ASSERT : bool = { if BS :: USIZE == 0 { panic ! ("Block size can not be equal to zero!") ; } if BS :: USIZE > 255 { panic ! ("Block size can not be bigger than 255!") ; } true } ; }
};
}
