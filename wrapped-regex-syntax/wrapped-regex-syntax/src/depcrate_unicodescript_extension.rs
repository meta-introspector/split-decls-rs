// Generated macro for script_extension (function)
macro_rules! Depcrate_unicodescript_extension {
() => {
// Module: crate::unicode
// Provides: {"script_extension"}
// Dependencies: {}
# [doc = " Returns the Unicode HIR class corresponding to the given script extension."] # [doc = ""] # [doc = " Name canonicalization is assumed to be performed by the caller."] # [doc = ""] # [doc = " If the given script extension could not be found, or if the script data is"] # [doc = " not available, then an error is returned."] fn script_extension (canonical_name : & 'static str ,) -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-script"))] fn imp (_ : & 'static str) -> Result < hir :: ClassUnicode , Error > { Err (Error :: PropertyNotFound) } # [cfg (feature = "unicode-script")] fn imp (name : & 'static str) -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: script_extension :: BY_NAME ; property_set (BY_NAME , name) . map (hir_class) . ok_or (Error :: PropertyValueNotFound) } imp (canonical_name) }
};
}
