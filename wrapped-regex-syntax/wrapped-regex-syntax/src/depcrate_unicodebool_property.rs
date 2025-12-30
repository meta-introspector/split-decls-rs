// Generated macro for bool_property (function)
macro_rules! Depcrate_unicodebool_property {
() => {
// Module: crate::unicode
// Provides: {"bool_property"}
// Dependencies: {}
# [doc = " Returns the Unicode HIR class corresponding to the given Unicode boolean"] # [doc = " property."] # [doc = ""] # [doc = " Name canonicalization is assumed to be performed by the caller."] # [doc = ""] # [doc = " If the given boolean property could not be found, or if the boolean"] # [doc = " property data is not available, then an error is returned."] fn bool_property (canonical_name : & 'static str ,) -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-bool"))] fn imp (_ : & 'static str) -> Result < hir :: ClassUnicode , Error > { Err (Error :: PropertyNotFound) } # [cfg (feature = "unicode-bool")] fn imp (name : & 'static str) -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: property_bool :: BY_NAME ; property_set (BY_NAME , name) . map (hir_class) . ok_or (Error :: PropertyNotFound) } match canonical_name { "Decimal_Number" => perl_digit () , "White_Space" => perl_space () , name => imp (name) , } }
};
}
