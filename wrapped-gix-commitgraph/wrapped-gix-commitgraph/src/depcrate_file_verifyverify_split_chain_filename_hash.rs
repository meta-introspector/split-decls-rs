// Generated macro for verify_split_chain_filename_hash (function)
macro_rules! Depcrate_file_verifyverify_split_chain_filename_hash {
() => {
// Module: crate::file::verify
// Provides: {"verify_split_chain_filename_hash"}
// Dependencies: {}
# [doc = " If the given path's filename matches \"graph-{hash}.graph\", check that `hash` matches the"] # [doc = " expected hash."] fn verify_split_chain_filename_hash (path : & Path , expected : & gix_hash :: oid) -> Result < () , String > { path . file_name () . and_then (std :: ffi :: OsStr :: to_str) . and_then (| filename | filename . strip_suffix (".graph")) . and_then (| stem | stem . strip_prefix ("graph-")) . map_or (Ok (()) , | hex | match gix_hash :: ObjectId :: from_hex (hex . as_bytes ()) { Ok (actual) if actual == expected => Ok (()) , _ => Err (format ! ("graph-{}.graph" , expected . to_hex ())) , }) }
};
}
