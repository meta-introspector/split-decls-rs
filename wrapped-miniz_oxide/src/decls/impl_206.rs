macro_rules! deps {
    () => {
        DecompressError!();
        TINFLStatus!();
        Result!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        # [cfg (feature = "with-alloc")] impl alloc :: fmt :: Display for DecompressError { # [cold] fn fmt (& self , f : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { f . write_str (match self . status { TINFLStatus :: FailedCannotMakeProgress => "Truncated input stream" , TINFLStatus :: BadParam => "Invalid output buffer size" , TINFLStatus :: Adler32Mismatch => "Adler32 checksum mismatch" , TINFLStatus :: Failed => "Invalid input data" , TINFLStatus :: Done => "" , TINFLStatus :: NeedsMoreInput => "Truncated input stream" , TINFLStatus :: HasMoreOutput => "Output size exceeded the specified limit" , # [cfg (feature = "block-boundary")] TINFLStatus :: BlockBoundary => "Reached end of a deflate block" , }) } }
    };
}

impl_206!();