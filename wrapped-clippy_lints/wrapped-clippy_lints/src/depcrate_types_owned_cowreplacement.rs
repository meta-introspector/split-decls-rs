// Generated macro for replacement (function)
macro_rules! Depcrate_types_owned_cowreplacement {
() => {
// Module: crate::types::owned_cow
// Provides: {"replacement"}
// Dependencies: {}
fn replacement (cx : & LateContext < '_ > , cty : & hir :: Ty < '_ >) -> Option < (Span , String) > { if cty . basic_res () . is_lang_item (cx , hir :: LangItem :: String) { return Some ((cty . span , "str" . into ())) ; } if cty . basic_res () . is_diag_item (cx , sym :: Vec) { return if let hir :: TyKind :: Path (hir :: QPath :: Resolved (_ , path)) = cty . kind && let [.. , last_seg] = path . segments && let Some (args) = last_seg . args && let [t , ..] = args . args && let Some (snip) = snippet_opt (cx , t . span ()) { Some ((cty . span , format ! ("[{snip}]"))) } else { None } ; } if cty . basic_res () . is_diag_item (cx , sym :: cstring_type) { return Some ((cty . span , (if clippy_utils :: is_no_std_crate (cx) { "core::ffi::CStr" } else { "std::ffi::CStr" }) . into () ,)) ; } for (diag , repl) in [(sym :: OsString , "std::ffi::OsStr") , (sym :: PathBuf , "std::path::Path")] { if cty . basic_res () . is_diag_item (cx , diag) { return Some ((cty . span , repl . into ())) ; } } None }
};
}
