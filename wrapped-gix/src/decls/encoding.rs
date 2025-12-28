macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! encoding {
    () => {
        deps!();
        # [doc = ""] pub mod encoding { use crate :: bstr :: BString ; # [doc = " The error produced when failing to parse the `core.checkRoundTripEncoding` key."] # [derive (Debug , thiserror :: Error)] # [error ("The encoding named '{encoding}' seen in key '{key}={value}' is unsupported")] pub struct Error { # [doc = " The configuration key that contained the value."] pub key : BString , # [doc = " The value that was assigned to `key`."] pub value : BString , # [doc = " The encoding that failed."] pub encoding : BString , } }
    };
}

encoding!();