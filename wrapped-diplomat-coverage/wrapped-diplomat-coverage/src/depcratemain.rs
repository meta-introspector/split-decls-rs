// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let out_path = std :: env :: args () . nth (1) ; let doc_types = ["icu" , "fixed_decimal" , "icu_provider_adapters"] . into_iter () . flat_map (collect_public_types) . map (| (path_vec , typ) | { let mut path = ast :: Path :: empty () ; path . elements = path_vec . into_iter () . map (ast :: Ident :: from) . collect () ; RustLinkInfo { path , typ } }) . filter (| rl | { ! [ast :: DocType :: EnumVariant , ast :: DocType :: Mod , ast :: DocType :: Trait ,] . contains (& rl . typ) }) . collect :: < BTreeSet < _ > > () ; let capi_crate = PathBuf :: from (concat ! (std :: env ! ("CARGO_MANIFEST_DIR") , "/../../../ffi/capi/src/lib.rs")) ; eprintln ! ("Loading icu_capi crate from {capi_crate:?}") ; let capi_types = ast :: File :: from (& syn_inline_mod :: parse_and_inline_modules (& capi_crate)) . all_rust_links () . into_iter () . cloned () . map (| rl | RustLinkInfo { path : rl . path , typ : rl . typ , }) . collect :: < BTreeSet < _ > > () ; let mut file_anchor = None ; let mut stdout_anchor = None ; let out_stream = if let Some (out_path) = out_path { let stream = file_anchor . insert (File :: create (out_path) . expect ("opening output file")) ; stream as & mut dyn std :: io :: Write } else { let stream = stdout_anchor . insert (std :: io :: stdout ()) ; stream as & mut dyn std :: io :: Write } ; writeln ! (out_stream , "{FILE_HEADER}") . unwrap () ; doc_types . difference (& capi_types) . for_each (| item | writeln ! (out_stream , "{item}") . unwrap ()) ; }
};
}
