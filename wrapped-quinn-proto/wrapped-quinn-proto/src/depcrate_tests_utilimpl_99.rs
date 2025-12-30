// Generated macro for impl_99 (impl)
macro_rules! Depcrate_tests_utilimpl_99 {
() => {
// Module: crate::tests::util
// Provides: {"impl_99"}
// Dependencies: {}
impl Write for TestWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { print ! ("{}" , str :: from_utf8 (buf) . expect ("tried to log invalid UTF-8")) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { io :: stdout () . flush () } }
};
}
