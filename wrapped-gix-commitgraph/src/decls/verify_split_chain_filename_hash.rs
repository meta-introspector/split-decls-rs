macro_rules! verify_split_chain_filename_hash {
    () => {
        # [doc = " If the given path's filename matches \"graph-{hash}.graph\", check that `hash` matches the"] # [doc = " expected hash."] fn verify_split_chain_filename_hash (path : & Path , expected : & gix_hash :: oid) -> Result < () , String > { path . file_name () . and_then (std :: ffi :: OsStr :: to_str) . and_then (| filename | filename . strip_suffix (".graph")) . and_then (| stem | stem . strip_prefix ("graph-")) . map_or (Ok (()) , | hex | match gix_hash :: ObjectId :: from_hex (hex . as_bytes ()) { Ok (actual) if actual == expected => Ok (()) , _ => Err (format ! ("graph-{}.graph" , expected . to_hex ())) , }) }
    };
}

verify_split_chain_filename_hash!();