// Generated macro for includemod (function)
macro_rules! Depcrateincludemod {
() => {
// Module: crate
// Provides: {"includemod"}
// Dependencies: {}
# [doc = " Macro to wrap includes in modules with proper imports"] # [doc = " "] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " includemod!(my_module, \"path/to/file.rs\");"] # [doc = " ```"] # [proc_macro] pub fn includemod (input : TokenStream) -> TokenStream { let input = input . to_string () ; let parts : Vec < & str > = input . split (',') . map (| s | s . trim ()) . collect () ; if parts . len () != 2 { panic ! ("includemod! expects exactly 2 arguments: module_name, file_path") ; } let mod_name = parts [0] ; let file_path = parts [1] . trim_matches ('"') ; let mod_ident = syn :: parse_str :: < Ident > (mod_name) . expect ("Invalid module name") ; let expanded = quote ! { pub mod # mod_ident { use std :: path :: { Path , PathBuf } ; use anyhow :: Result ; use super :: split_decls_config_mod :: SplitDeclsConfig ; macro_rules ! mkdeclfn { (fn $ name : ident $ ($ rest : tt) *) => { pub fn $ name $ ($ rest) * } ; } include ! (# file_path) ; } } ; TokenStream :: from (expanded) }
};
}
