macro_rules! Error {
    () => {
        # [doc = " Indicates key or values contain errors that can't be encoded."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{key:?}={value:?} must not contain null bytes or newlines neither in key nor in value.")] Encoding { key : String , value : BString } , }
    };
}

Error!();