// Generated macro for impl_131 (impl)
macro_rules! Depcrate_readerimpl_131 {
() => {
// Module: crate::reader
// Provides: {"impl_131"}
// Dependencies: {}
impl < R : io :: Read > Iterator for StringRecordsIntoIter < R > { type Item = Result < StringRecord > ; fn next (& mut self) -> Option < Result < StringRecord > > { match self . rdr . read_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (true) => Some (Ok (self . rec . clone_truncated ())) , Ok (false) => None , } } }
};
}
