// Generated macro for parse_file (function)
macro_rules! Depcrate_indexparse_file {
() => {
// Module: crate::index
// Provides: {"parse_file"}
// Dependencies: {}
fn parse_file (index_path : impl AsRef < Path > , object_hash : gix :: hash :: Kind) -> anyhow :: Result < gix :: index :: File > { gix :: index :: File :: at (index_path . as_ref () , object_hash , false , Default :: default ()) . map_err (Into :: into) }
};
}
