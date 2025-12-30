// Generated macro for script (function)
macro_rules! Depcrate_unicodescript {
() => {
// Module: crate::unicode
// Provides: {"script"}
// Dependencies: {}
# [doc = " Returns the Unicode HIR class corresponding to the given script."] # [doc = ""] # [doc = " Name canonicalization is assumed to be performed by the caller."] # [doc = ""] # [doc = " If the given script could not be found, or if the script data is not"] # [doc = " available, then an error is returned."] fn script (canonical_name : & 'static str) -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-script"))] fn imp (_ : & 'static str) -> Result < hir :: ClassUnicode , Error > { Err (Error :: PropertyNotFound) } # [cfg (feature = "unicode-script")] fn imp (name : & 'static str) -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: script :: BY_NAME ; property_set (BY_NAME , name) . map (hir_class) . ok_or (Error :: PropertyValueNotFound) } imp (canonical_name) }
};
}
