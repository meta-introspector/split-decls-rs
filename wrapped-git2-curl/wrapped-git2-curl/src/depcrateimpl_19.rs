// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Write for CurlSubtransport { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { if self . reader . is_none () { self . execute (data) ? ; } Ok (data . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
