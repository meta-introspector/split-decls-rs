// Generated macro for impl_101 (impl)
macro_rules! Depcrate_progressimpl_101 {
() => {
// Module: crate::progress
// Provides: {"impl_101"}
// Dependencies: {}
impl < T , P > io :: Write for Write < T , P > where T : io :: Write , P : Progress , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let written = self . inner . write (buf) ? ; self . progress . inc_by (written) ; Ok (written) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
