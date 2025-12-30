// Generated macro for impl_140 (impl)
macro_rules! Depcrate_readerimpl_140 {
() => {
// Module: crate::reader
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'r , R : io :: Read > Iterator for ByteRecordsIter < 'r , R > { type Item = Result < ByteRecord > ; fn next (& mut self) -> Option < Result < ByteRecord > > { match self . rdr . read_byte_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (true) => Some (Ok (self . rec . clone_truncated ())) , Ok (false) => None , } } }
};
}
