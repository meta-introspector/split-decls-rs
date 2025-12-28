macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! canonical_prop {
    () => {
        deps!();
        # [doc = " Find the canonical property name for the given normalized property name."] # [doc = ""] # [doc = " If no such property exists, then `None` is returned."] # [doc = ""] # [doc = " The normalized property name must have been normalized according to"] # [doc = " UAX44 LM3, which can be done using `symbolic_name_normalize`."] # [doc = ""] # [doc = " If the property names data is not available, then an error is returned."] fn canonical_prop (normalized_name : & str ,) -> Result < Option < & 'static str > , Error > { # [cfg (not (any (feature = "unicode-age" , feature = "unicode-bool" , feature = "unicode-gencat" , feature = "unicode-perl" , feature = "unicode-script" , feature = "unicode-segment" ,)))] fn imp (_ : & str) -> Result < Option < & 'static str > , Error > { Err (Error :: PropertyNotFound) } # [cfg (any (feature = "unicode-age" , feature = "unicode-bool" , feature = "unicode-gencat" , feature = "unicode-perl" , feature = "unicode-script" , feature = "unicode-segment" ,))] fn imp (name : & str) -> Result < Option < & 'static str > , Error > { use crate :: unicode_tables :: property_names :: PROPERTY_NAMES ; Ok (PROPERTY_NAMES . binary_search_by_key (& name , | & (n , _) | n) . ok () . map (| i | PROPERTY_NAMES [i] . 1)) } imp (normalized_name) }
    };
}

canonical_prop!()