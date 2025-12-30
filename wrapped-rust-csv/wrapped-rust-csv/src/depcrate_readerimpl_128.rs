// Generated macro for impl_128 (impl)
macro_rules! Depcrate_readerimpl_128 {
() => {
// Module: crate::reader
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'r , R : io :: Read , D : DeserializeOwned > Iterator for DeserializeRecordsIter < 'r , R , D > { type Item = Result < D > ; fn next (& mut self) -> Option < Result < D > > { match self . rdr . read_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (false) => None , Ok (true) => Some (self . rec . deserialize (self . headers . as_ref ())) , } } }
};
}
