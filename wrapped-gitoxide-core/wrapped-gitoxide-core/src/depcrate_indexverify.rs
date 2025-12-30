// Generated macro for verify (function)
macro_rules! Depcrate_indexverify {
() => {
// Module: crate::index
// Provides: {"verify"}
// Dependencies: {}
pub fn verify (index_path : impl AsRef < Path > , mut out : impl std :: io :: Write , Options { object_hash , format } : Options ,) -> anyhow :: Result < () > { let file = parse_file (index_path , object_hash) ? ; file . verify_integrity () ? ; file . verify_entries () ? ; file . verify_extensions (false , gix :: objs :: find :: Never) ? ; # [cfg_attr (not (feature = "serde") , allow (irrefutable_let_patterns))] if let crate :: OutputFormat :: Human = format { writeln ! (out , "OK") . ok () ; } Ok (()) }
};
}
