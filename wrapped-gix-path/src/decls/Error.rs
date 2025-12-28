macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error used in [`RelativePath`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("A RelativePath is not allowed to be absolute")] IsAbsolute , # [error (transparent)] ContainsInvalidComponent (# [from] gix_validate :: path :: component :: Error) , # [error (transparent)] IllegalUtf8 (# [from] crate :: Utf8Error) , }
    };
}

Error!();