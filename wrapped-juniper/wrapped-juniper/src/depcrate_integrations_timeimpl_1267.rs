// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_integrations_timeimpl_1267 {
() => {
// Module: crate::integrations::time
// Provides: {"impl_1267"}
// Dependencies: {}
impl io :: Write for IoAdapter < '_ , '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let s = str :: from_utf8 (buf) . map_err (io :: Error :: other) ? ; match self . 0 . write_str (s) { Ok (_) => Ok (s . len ()) , Err (e) => Err (io :: Error :: other (e)) , } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
