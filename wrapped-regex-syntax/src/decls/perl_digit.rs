macro_rules! deps {
    () => {
        Error!();
        ClassUnicode!();
        Result!();
    };
}

macro_rules! perl_digit {
    () => {
        deps!();
        # [doc = " Returns a Unicode aware class for \\d."] # [doc = ""] # [doc = " This returns an error if the data is not available for \\d."] pub fn perl_digit () -> Result < hir :: ClassUnicode , Error > { # [cfg (not (any (feature = "unicode-perl" , feature = "unicode-gencat")))] fn imp () -> Result < hir :: ClassUnicode , Error > { Err (Error :: PerlClassNotFound) } # [cfg (all (feature = "unicode-perl" , not (feature = "unicode-gencat")))] fn imp () -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: perl_decimal :: DECIMAL_NUMBER ; Ok (hir_class (DECIMAL_NUMBER)) } # [cfg (feature = "unicode-gencat")] fn imp () -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: general_category :: DECIMAL_NUMBER ; Ok (hir_class (DECIMAL_NUMBER)) } imp () }
    };
}

perl_digit!();