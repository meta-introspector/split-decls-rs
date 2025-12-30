// Generated macro for impl_106 (impl)
macro_rules! Depcrate_boxedimpl_106 {
() => {
// Module: crate::boxed
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R : std :: io :: Read + ? Sized , A : Allocator > std :: io :: Read for Box < R , A > { # [inline] fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { (* * self) . read (buf) } # [inline] fn read_to_end (& mut self , buf : & mut std :: vec :: Vec < u8 >) -> std :: io :: Result < usize > { (* * self) . read_to_end (buf) } # [inline] fn read_to_string (& mut self , buf : & mut String) -> std :: io :: Result < usize > { (* * self) . read_to_string (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> std :: io :: Result < () > { (* * self) . read_exact (buf) } }
};
}
