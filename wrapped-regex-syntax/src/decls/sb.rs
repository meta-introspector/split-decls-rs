macro_rules! deps {
    () => {
        Result!();
        Error!();
        ClassUnicode!();
    };
}

macro_rules! sb {
    () => {
        deps!();
        # [doc = " Returns the Unicode HIR class corresponding to the given sentence"] # [doc = " break property."] # [doc = ""] # [doc = " Name canonicalization is assumed to be performed by the caller."] # [doc = ""] # [doc = " If the given property could not be found, or if the corresponding data is"] # [doc = " not available, then an error is returned."] fn sb (canonical_name : & 'static str) -> Result < hir :: ClassUnicode , Error > { # [cfg (not (feature = "unicode-segment"))] fn imp (_ : & 'static str) -> Result < hir :: ClassUnicode , Error > { Err (Error :: PropertyNotFound) } # [cfg (feature = "unicode-segment")] fn imp (name : & 'static str) -> Result < hir :: ClassUnicode , Error > { use crate :: unicode_tables :: sentence_break :: BY_NAME ; property_set (BY_NAME , name) . map (hir_class) . ok_or (Error :: PropertyValueNotFound) } imp (canonical_name) }
    };
}

sb!()