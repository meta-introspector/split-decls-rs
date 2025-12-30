// Generated macro for impl_49 (impl)
macro_rules! Depcrate_termimpl_49 {
() => {
// Module: crate::term
// Provides: {"impl_49"}
// Dependencies: {}
impl Write for & Term { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match self . inner . buffer { Some (ref buffer) => buffer . lock () . unwrap () . write_all (buf) , None => self . write_through (buf) , } ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Term :: flush (self) } }
};
}
