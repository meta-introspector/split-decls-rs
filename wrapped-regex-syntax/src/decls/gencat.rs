macro_rules! deps {
    () => {
        Result!();
        ClassUnicode!();
        Error!();
    };
}

macro_rules! gencat {
    () => {
        deps!();
        # [doc = " Returns the Unicode HIR class corresponding to the given general category."] # [doc = ""] # [doc = " Name canonicalization is assumed to be performed by the caller."] # [doc = ""] # [doc = " If the given general category could not be found, or if the general"] # [doc = " category data is not available, then an error is returned."] fn gencat (canonical_name : & 'static str) -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-gencat"))] fn imp (_ : & 'static str) -> Result < hir :: ClassUnicode , Error > { Err (Error :: PropertyNotFound) } # [cfg (feature = "unicode-gencat")] fn imp (name : & 'static str) -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: general_category :: BY_NAME ; match name { "ASCII" => Ok (hir_class (& [('\0' , '\x7F')])) , "Any" => Ok (hir_class (& [('\0' , '\u{10FFFF}')])) , "Assigned" => { let mut cls = gencat ("Unassigned") ? ; cls . negate () ; Ok (cls) } name => property_set (BY_NAME , name) . map (hir_class) . ok_or (Error :: PropertyValueNotFound) , } } match canonical_name { "Decimal_Number" => perl_digit () , name => imp (name) , } }
    };
}

gencat!();