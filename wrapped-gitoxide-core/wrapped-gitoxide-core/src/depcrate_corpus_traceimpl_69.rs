// Generated macro for impl_69 (impl)
macro_rules! Depcrate_corpus_traceimpl_69 {
() => {
// Module: crate::corpus::trace
// Provides: {"impl_69"}
// Dependencies: {}
impl tracing_forest :: printer :: Formatter for StoreTreeToDb { type Error = rusqlite :: Error ; fn fmt (& self , tree : & Tree) -> Result < String , Self :: Error > { if let Some ((progress , tree)) = self . progress . as_ref () . map (Mutex :: lock) . zip (tracing_forest :: printer :: Pretty . fmt (tree) . ok ()) { use gix :: Progress ; if self . reverse_lines { for line in tree . lines () . rev () { progress . info (line . into ()) ; } } else { for line in tree . lines () { progress . info (line . into ()) ; } } } let json = serde_json :: to_string_pretty (& tree) . expect ("serialization to string always works") ; let run_id = self . run_id . load (Ordering :: SeqCst) ; self . con . lock () . execute ("UPDATE run SET spans_json = ?1 WHERE id = ?2" , params ! [json , run_id]) ? ; Ok (String :: new ()) } }
};
}
