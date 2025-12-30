// Generated macro for impl_96 (impl)
macro_rules! Depcrate_readimpl_96 {
() => {
// Module: crate::read
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R > io :: Read for OffsetReader < R > where R : io :: Read , { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let r = self . reader . read (buf) ; if let Ok (count) = r { self . offset += count as u64 ; } r } }
};
}
