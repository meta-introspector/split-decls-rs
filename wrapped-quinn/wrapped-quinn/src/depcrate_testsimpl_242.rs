// Generated macro for impl_242 (impl)
macro_rules! Depcrate_testsimpl_242 {
() => {
// Module: crate::tests
// Provides: {"impl_242"}
// Dependencies: {}
impl std :: io :: Write for TestWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { print ! ("{}" , str :: from_utf8 (buf) . expect ("tried to log invalid UTF-8")) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { io :: stdout () . flush () } }
};
}
