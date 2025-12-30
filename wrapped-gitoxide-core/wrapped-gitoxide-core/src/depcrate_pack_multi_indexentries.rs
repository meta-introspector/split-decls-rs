// Generated macro for entries (function)
macro_rules! Depcrate_pack_multi_indexentries {
() => {
// Module: crate::pack::multi_index
// Provides: {"entries"}
// Dependencies: {}
pub fn entries (multi_index_path : PathBuf , format : OutputFormat , mut out : impl std :: io :: Write) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("Only human format is supported right now") ; } let file = gix :: odb :: pack :: multi_index :: File :: at (multi_index_path) ? ; for entry in file . iter () { writeln ! (out , "{} {} {}" , entry . oid , entry . pack_index , entry . pack_offset) ? ; } Ok (()) }
};
}
