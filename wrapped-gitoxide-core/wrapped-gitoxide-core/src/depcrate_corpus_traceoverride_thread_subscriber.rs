// Generated macro for override_thread_subscriber (function)
macro_rules! Depcrate_corpus_traceoverride_thread_subscriber {
() => {
// Module: crate::corpus::trace
// Provides: {"override_thread_subscriber"}
// Dependencies: {}
pub fn override_thread_subscriber (db_path : impl AsRef < Path > , progress : Option < ProgressItem > , reverse_lines : bool ,) -> anyhow :: Result < (tracing :: subscriber :: DefaultGuard , Arc < AtomicU32 >) > { let current_id = Arc :: new (AtomicU32 :: default ()) ; let processor = tracing_forest :: Printer :: new () . formatter (StoreTreeToDb { con : Arc :: new (Mutex :: new (rusqlite :: Connection :: open (& db_path) ?)) , run_id : current_id . clone () , progress : progress . map (Mutex :: new) , reverse_lines , }) ; let subscriber = tracing_subscriber :: Registry :: default () . with (tracing_forest :: ForestLayer :: from (processor)) ; let guard = tracing :: subscriber :: set_default (subscriber) ; Ok ((guard , current_id)) }
};
}
