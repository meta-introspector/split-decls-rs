// Generated macro for impl_137 (impl)
macro_rules! Depcrate_readerimpl_137 {
() => {
// Module: crate::reader
// Provides: {"impl_137"}
// Dependencies: {}
impl < R : io :: Read > Iterator for ByteRecordsIntoIter < R > { type Item = Result < ByteRecord > ; fn next (& mut self) -> Option < Result < ByteRecord > > { match self . rdr . read_byte_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (true) => Some (Ok (self . rec . clone_truncated ())) , Ok (false) => None , } } }
};
}
