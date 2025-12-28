macro_rules! CompressError {
    () => {
        # [doc = " The error produced by [`Compress::compress()`]."] # [derive (Debug , thiserror :: Error)] # [error ("{msg}")] # [allow (missing_docs)] pub enum CompressError { # [error ("stream error")] StreamError , # [error ("Not enough memory")] InsufficientMemory , # [error ("An unknown error occurred: {err}")] Unknown { err : c_int } , }
    };
}

CompressError!()