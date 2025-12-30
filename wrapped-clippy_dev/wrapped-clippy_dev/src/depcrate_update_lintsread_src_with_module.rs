// Generated macro for read_src_with_module (function)
macro_rules! Depcrate_update_lintsread_src_with_module {
() => {
// Module: crate::update_lints
// Provides: {"read_src_with_module"}
// Dependencies: {}
# [doc = " Reads the source files from the given root directory"] fn read_src_with_module (src_root : & Path) -> impl use < '_ > + Iterator < Item = (DirEntry , String) > { WalkDir :: new (src_root) . into_iter () . filter_map (move | e | { let e = expect_action (e , ErrAction :: Read , src_root) ; let path = e . path () . as_os_str () . as_encoded_bytes () ; if let Some (path) = path . strip_suffix (b".rs") && let Some (path) = path . get (src_root . as_os_str () . len () + 1 ..) { if path == b"lib" { Some ((e , String :: new ())) } else { let path = if let Some (path) = path . strip_suffix (b"mod") && let Some (path) = path . strip_suffix (b"/") . or_else (| | path . strip_suffix (b"\\")) { path } else { path } ; if let Ok (path) = str :: from_utf8 (path) { let path = path . replace (['/' , '\\'] , "::") ; Some ((e , path)) } else { None } } } else { None } }) }
};
}
