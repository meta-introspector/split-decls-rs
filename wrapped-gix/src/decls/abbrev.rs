macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! abbrev {
    () => {
        deps!();
        # [doc = ""] pub mod abbrev { use crate :: bstr :: BString ; # [doc = " The error describing an incorrect `core.abbrev` value."] # [derive (Debug , thiserror :: Error)] # [error ("Invalid value for 'core.abbrev' = '{}'. It must be between 4 and {}" , . value , . max)] pub struct Error { # [doc = " The value found in the git configuration"] pub value : BString , # [doc = " The maximum abbreviation length, the length of an object hash."] pub max : u8 , } }
    };
}

abbrev!();