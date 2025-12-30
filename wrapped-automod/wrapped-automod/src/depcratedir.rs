// Generated macro for dir (function)
macro_rules! Depcratedir {
() => {
// Module: crate
// Provides: {"dir"}
// Dependencies: {}
# [proc_macro] pub fn dir (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as Arg) ; let vis = & input . vis ; let rel_path = input . path . value () ; let dir = match env :: var_os ("CARGO_MANIFEST_DIR") { Some (manifest_dir) => PathBuf :: from (manifest_dir) . join (rel_path) , None => PathBuf :: from (rel_path) , } ; let expanded = match source_file_names (dir) { Ok (names) => names . into_iter () . map (| name | mod_item (vis , name)) . collect () , Err (err) => syn :: Error :: new (Span :: call_site () , err) . to_compile_error () , } ; TokenStream :: from (expanded) }
};
}
