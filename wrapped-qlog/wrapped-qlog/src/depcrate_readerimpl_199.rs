// Generated macro for impl_199 (impl)
macro_rules! Depcrate_readerimpl_199 {
() => {
// Module: crate::reader
// Provides: {"impl_199"}
// Dependencies: {}
impl Iterator for QlogSeqReader < '_ > { type Item = Event ; # [inline] fn next (& mut self) -> Option < Self :: Item > { while let Some (bytes) = Self :: read_record (& mut self . reader) { let r : serde_json :: Result < crate :: events :: Event > = serde_json :: from_slice (& bytes) ; if let Ok (event) = r { return Some (Event :: Qlog (event)) ; } let r : serde_json :: Result < crate :: events :: JsonEvent > = serde_json :: from_slice (& bytes) ; if let Ok (event) = r { return Some (Event :: Json (event)) ; } } None } }
};
}
