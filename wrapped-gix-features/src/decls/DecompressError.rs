macro_rules! DecompressError {
    () => {
        # [doc = " The error produced by [`Decompress::decompress()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum DecompressError { # [error ("stream error")] StreamError , # [error ("Not enough memory")] InsufficientMemory , # [error ("Invalid input data")] DataError , # [error ("An unknown error occurred: {err}")] Unknown { err : c_int } , }
    };
}

DecompressError!()