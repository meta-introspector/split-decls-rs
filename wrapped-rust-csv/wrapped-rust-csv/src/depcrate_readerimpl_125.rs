// Generated macro for impl_125 (impl)
macro_rules! Depcrate_readerimpl_125 {
() => {
// Module: crate::reader
// Provides: {"impl_125"}
// Dependencies: {}
impl < R : io :: Read , D : DeserializeOwned > Iterator for DeserializeRecordsIntoIter < R , D > { type Item = Result < D > ; fn next (& mut self) -> Option < Result < D > > { match self . rdr . read_record (& mut self . rec) { Err (err) => Some (Err (err)) , Ok (false) => None , Ok (true) => Some (self . rec . deserialize (self . headers . as_ref ())) , } } }
};
}
