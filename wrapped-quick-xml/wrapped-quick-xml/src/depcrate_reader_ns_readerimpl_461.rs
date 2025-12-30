// Generated macro for impl_461 (impl)
macro_rules! Depcrate_reader_ns_readerimpl_461 {
() => {
// Module: crate::reader::ns_reader
// Provides: {"impl_461"}
// Dependencies: {}
impl NsReader < BufReader < File > > { # [doc = " Creates an XML reader from a file path."] pub fn from_file < P : AsRef < Path > > (path : P) -> Result < Self > { Ok (Self :: new (Reader :: from_file (path) ?)) } }
};
}
