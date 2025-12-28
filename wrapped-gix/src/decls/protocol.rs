macro_rules! deps {
    () => {
        Error!();
        String!();
    };
}

macro_rules! protocol {
    () => {
        deps!();
        # [doc = ""] pub mod protocol { # [doc = ""] pub mod allow { use crate :: bstr :: BString ; # [doc = " The error returned when obtaining the permission for a particular scheme."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error ("The value {value:?} must be allow|deny|user in configuration key protocol{0}.allow" , scheme . as_ref () . map (| s | format ! (".{s}")) . unwrap_or_default ())] pub struct Error { pub scheme : Option < String > , pub value : BString , } } }
    };
}

protocol!()