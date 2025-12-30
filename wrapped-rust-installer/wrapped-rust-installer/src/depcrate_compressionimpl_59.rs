// Generated macro for impl_59 (impl)
macro_rules! Depcrate_compressionimpl_59 {
() => {
// Module: crate::compression
// Provides: {"impl_59"}
// Dependencies: {}
impl Write for CombinedEncoder { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . write_all (buf) ? ; Ok (buf . len ()) } fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { self . encoders . par_iter_mut () . try_for_each (| w | w . write_all (buf)) } fn flush (& mut self) -> std :: io :: Result < () > { self . encoders . par_iter_mut () . try_for_each (Write :: flush) } }
};
}
