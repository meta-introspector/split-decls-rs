macro_rules! deps {
    () => {
        CompressorOxide!();
        Error!();
    };
}

macro_rules! TDEFLStatus {
    () => {
        deps!();
        # [doc = " Return status of compression."] # [repr (i32)] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum TDEFLStatus { # [doc = " Usage error."] # [doc = ""] # [doc = " This indicates that either the [`CompressorOxide`] experienced a previous error, or the"] # [doc = " stream has already been [`TDEFLFlush::Finish`]'d."] BadParam = - 2 , # [doc = " Error putting data into output buffer."] # [doc = ""] # [doc = " This usually indicates a too-small buffer."] PutBufFailed = - 1 , # [doc = " Compression succeeded normally."] Okay = 0 , # [doc = " Compression succeeded and the deflate stream was ended."] # [doc = ""] # [doc = " This is the result of calling compression with [`TDEFLFlush::Finish`]."] Done = 1 , }
    };
}

TDEFLStatus!();