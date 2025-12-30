// Generated macro for impl_133 (impl)
macro_rules! Depcrateimpl_133 {
() => {
// Module: crate
// Provides: {"impl_133"}
// Dependencies: {}
impl Include { fn from_path_only (path : & str) -> Result < Include , String > { const NEEDLE : & str = "zoneinfo/" ; let Some (zoneinfo) = path . rfind (NEEDLE) else { return Err (format ! ("could not extract IANA time zone identifier from \
                 file path `{path}` \
                 (could not find `zoneinfo` in path), \
                 please provide IANA time zone identifier as second \
                 parameter" ,)) ; } ; let idstart = zoneinfo . saturating_add (NEEDLE . len ()) ; let id = & path [idstart ..] ; Include :: from_path_with_id (id , path) } fn from_path_with_id (id : & str , path : & str) -> Result < Include , String > { let id = id . to_string () ; let data = std :: fs :: read (path) . map_err (| e | format ! ("failed to read {path}: {e}")) ? ; let tzif = TzifOwned :: parse (Some (id . clone ()) , & data) . map_err (| e | { format ! ("failed to parse TZif data from {path}: {e}") }) ? ; Ok (Include { tzif }) } fn quote (& self) -> proc_macro2 :: TokenStream { self . tzif . quote () } }
};
}
