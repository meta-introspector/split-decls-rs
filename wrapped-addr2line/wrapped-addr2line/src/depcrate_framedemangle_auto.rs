// Generated macro for demangle_auto (function)
macro_rules! Depcrate_framedemangle_auto {
() => {
// Module: crate::frame
// Provides: {"demangle_auto"}
// Dependencies: {}
# [doc = " Apply 'best effort' demangling of a symbol name."] # [doc = ""] # [doc = " If `language` is given, then only the demangling scheme for that language"] # [doc = " is used."] # [doc = ""] # [doc = " If `language` is `None`, then heuristics are used to determine how to"] # [doc = " demangle the name. Currently, these heuristics are very basic."] # [doc = ""] # [doc = " If demangling fails or is not required, then `name` is returned unchanged."] pub fn demangle_auto (name : Cow < '_ , str > , language : Option < gimli :: DwLang >) -> Cow < '_ , str > { match language { Some (language) => demangle (name . as_ref () , language) , None => demangle (name . as_ref () , gimli :: DW_LANG_Rust) . or_else (| | demangle (name . as_ref () , gimli :: DW_LANG_C_plus_plus)) , } . map (Cow :: from) . unwrap_or (name) }
};
}
