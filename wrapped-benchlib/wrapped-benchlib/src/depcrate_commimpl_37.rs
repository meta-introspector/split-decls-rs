// Generated macro for impl_37 (impl)
macro_rules! Depcrate_commimpl_37 {
() => {
// Module: crate::comm
// Provides: {"impl_37"}
// Dependencies: {}
impl < R : Read > Iterator for MessageReader < R > { type Item = anyhow :: Result < BenchmarkMessage > ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . read_line (& mut self . line) { Ok (0) => None , Ok (_) => match serde_json :: from_str (& self . line) { Ok (value) => Some (Ok (value)) , Err (error) => Some (Err (error . into ())) , } , Err (error) => Some (Err (error . into ())) , } } }
};
}
