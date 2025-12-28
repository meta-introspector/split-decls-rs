macro_rules! Status {
    () => {
        # [doc = " The status returned by [`Decompress::decompress()`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Status { # [doc = " The decompress operation went well. Not to be confused with `StreamEnd`, so one can continue"] # [doc = " the decompression."] Ok , # [doc = " An error occurred when decompression."] BufError , # [doc = " The stream was fully decompressed."] StreamEnd , }
    };
}

Status!();