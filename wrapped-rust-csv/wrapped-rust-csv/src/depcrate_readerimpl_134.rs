// Generated macro for impl_134 (impl)
macro_rules! Depcrate_readerimpl_134 {
() => {
// Module: crate::reader
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'r , R : io :: Read > Iterator for StringRecordsIter < 'r , R > { type Item = Result < StringRecord > ; fn next (& mut self) -> Option < Result < StringRecord > > { match self . rdr . read_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (true) => Some (Ok (self . rec . clone_truncated ())) , Ok (false) => None , } } }
};
}
