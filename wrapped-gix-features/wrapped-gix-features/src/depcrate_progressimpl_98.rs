// Generated macro for impl_98 (impl)
macro_rules! Depcrate_progressimpl_98 {
() => {
// Module: crate::progress
// Provides: {"impl_98"}
// Dependencies: {}
impl < T , P > io :: Read for Read < T , P > where T : io :: Read , P : Progress , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes_read = self . inner . read (buf) ? ; self . progress . inc_by (bytes_read) ; Ok (bytes_read) } }
};
}
