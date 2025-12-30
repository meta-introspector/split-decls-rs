// Generated macro for StoreTreeToDb (struct)
macro_rules! Depcrate_corpus_traceStoreTreeToDb {
() => {
// Module: crate::corpus::trace
// Provides: {"StoreTreeToDb"}
// Dependencies: {}
pub struct StoreTreeToDb { con : Arc < Mutex < rusqlite :: Connection > > , run_id : Arc < AtomicU32 > , progress : Option < Mutex < ProgressItem > > , reverse_lines : bool , }
};
}
