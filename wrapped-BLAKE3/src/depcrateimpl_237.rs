// Generated macro for impl_237 (impl)
macro_rules! Depcrateimpl_237 {
() => {
// Module: crate
// Provides: {"impl_237"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: io :: Read for OutputReader { # [inline] fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { self . fill (buf) ; Ok (buf . len ()) } }
};
}
