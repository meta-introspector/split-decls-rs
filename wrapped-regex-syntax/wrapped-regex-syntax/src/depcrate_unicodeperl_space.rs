// Generated macro for perl_space (function)
macro_rules! Depcrate_unicodeperl_space {
() => {
// Module: crate::unicode
// Provides: {"perl_space"}
// Dependencies: {}
# [doc = " Returns a Unicode aware class for \\s."] # [doc = ""] # [doc = " This returns an error if the data is not available for \\s."] pub fn perl_space () -> Result < hir :: ClassUnicode , Error > { # [cfg (not (any (feature = "unicode-perl" , feature = "unicode-bool")))] fn imp () -> Result < hir :: ClassUnicode , Error > { Err (Error :: PerlClassNotFound) } # [cfg (all (feature = "unicode-perl" , not (feature = "unicode-bool")))] fn imp () -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: perl_space :: WHITE_SPACE ; Ok (hir_class (WHITE_SPACE)) } # [cfg (feature = "unicode-bool")] fn imp () -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: property_bool :: WHITE_SPACE ; Ok (hir_class (WHITE_SPACE)) } imp () }
};
}
