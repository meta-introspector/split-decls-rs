// Generated macro for impl_444 (impl)
macro_rules! Depcrate_reader_buffered_readerimpl_444 {
() => {
// Module: crate::reader::buffered_reader
// Provides: {"impl_444"}
// Dependencies: {}
impl Reader < BufReader < File > > { # [doc = " Creates an XML reader from a file path."] pub fn from_file < P : AsRef < Path > > (path : P) -> Result < Self > { let file = File :: open (path) ? ; let reader = BufReader :: new (file) ; Ok (Self :: from_reader (reader)) } }
};
}
