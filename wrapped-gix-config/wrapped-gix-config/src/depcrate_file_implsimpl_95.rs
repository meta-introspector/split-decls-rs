// Generated macro for impl_95 (impl)
macro_rules! Depcrate_file_implsimpl_95 {
() => {
// Module: crate::file::impls
// Provides: {"impl_95"}
// Dependencies: {}
impl FromStr for File < 'static > { type Err = parse :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { parse :: Events :: from_bytes_owned (s . as_bytes () , None) . map (| events | File :: from_parse_events_no_includes (events , Metadata :: api ())) } }
};
}
