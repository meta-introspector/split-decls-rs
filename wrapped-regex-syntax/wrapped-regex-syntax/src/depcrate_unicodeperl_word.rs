// Generated macro for perl_word (function)
macro_rules! Depcrate_unicodeperl_word {
() => {
// Module: crate::unicode
// Provides: {"perl_word"}
// Dependencies: {}
# [doc = " Returns a Unicode aware class for \\w."] # [doc = ""] # [doc = " This returns an error if the data is not available for \\w."] pub fn perl_word () -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-perl"))] fn imp () -> Result < hir :: ClassUnicode , Error > { Err (Error :: PerlClassNotFound) } # [cfg (feature = "unicode-perl")] fn imp () -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: perl_word :: PERL_WORD ; Ok (hir_class (PERL_WORD)) } imp () }
};
}
