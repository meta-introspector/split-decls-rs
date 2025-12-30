// Generated macro for impl_823 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_823 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_823"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : ? Sized > io :: Read for Retained < T > where for < 'a > & 'a T : io :: Read , { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (& * * self) . read (buf) } # [inline] fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { (& * * self) . read_vectored (bufs) } # [inline] fn read_to_end (& mut self , buf : & mut std :: vec :: Vec < u8 >) -> io :: Result < usize > { (& * * self) . read_to_end (buf) } # [inline] fn read_to_string (& mut self , buf : & mut std :: string :: String) -> io :: Result < usize > { (& * * self) . read_to_string (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { (& * * self) . read_exact (buf) } }
};
}
