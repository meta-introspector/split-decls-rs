// Generated macro for info (function)
macro_rules! Depcrate_pack_multi_indexinfo {
() => {
// Module: crate::pack::multi_index
// Provides: {"info"}
// Dependencies: {}
# [cfg_attr (not (feature = "serde") , allow (unused_variables))] pub fn info (multi_index_path : PathBuf , format : OutputFormat , out : impl std :: io :: Write , mut err : impl std :: io :: Write ,) -> anyhow :: Result < () > { if format == OutputFormat :: Human { writeln ! (err , "Defaulting to JSON as human format isn't implemented") . ok () ; } # [cfg (feature = "serde")] { let file = gix :: odb :: pack :: multi_index :: File :: at (& multi_index_path) ? ; serde_json :: to_writer_pretty (out , & info :: Statistics { path : multi_index_path , num_objects : file . num_objects () , index_names : file . index_names () . to_vec () , object_hash : file . object_hash () . to_string () , } ,) ? ; } Ok (()) }
};
}
