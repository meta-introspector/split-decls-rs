macro_rules! error {
    () => {
        mod error { use bstr :: BString ; # [doc = " The error returned by [`parse()`][crate::parse()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Line {line_number} has too many names or emails, or none at all: {line:?}")] UnconsumedInput { line_number : usize , line : BString } , # [error ("{line_number}: {line:?}: {message}")] Malformed { line_number : usize , line : BString , message : String , } , } }
    };
}

error!()