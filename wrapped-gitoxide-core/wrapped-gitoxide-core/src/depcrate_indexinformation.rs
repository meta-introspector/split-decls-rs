// Generated macro for information (function)
macro_rules! Depcrate_indexinformation {
() => {
// Module: crate::index
// Provides: {"information"}
// Dependencies: {}
# [cfg_attr (not (feature = "serde") , allow (unused_variables , unused_mut))] pub fn information (index_path : impl AsRef < Path > , out : impl std :: io :: Write , mut err : impl std :: io :: Write , information :: Options { index : Options { object_hash , mut format , } , extension_details , } : information :: Options ,) -> anyhow :: Result < () > { use crate :: OutputFormat :: * ; # [cfg (feature = "serde")] if let Human = format { writeln ! (err , "Defaulting to JSON printing as nothing else will be implemented.") . ok () ; format = Json ; } match format { Human => { anyhow :: bail ! ("Cannot print information using 'human' format.") } # [cfg (feature = "serde")] Json => { let info = information :: Collection :: try_from_file (parse_file (index_path , object_hash) ? , extension_details) ? ; serde_json :: to_writer_pretty (out , & info) ? ; Ok (()) } } }
};
}
