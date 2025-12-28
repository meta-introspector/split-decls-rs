macro_rules! CompressionFormat {
    () => {
        # [doc = " A data compression format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum CompressionFormat { # [doc = " The data is uncompressed."] None , # [doc = " The data is compressed, but the compression format is unknown."] Unknown , # [doc = " ZLIB/DEFLATE."] # [doc = ""] # [doc = " Used for ELF compression and GNU compressed debug information."] Zlib , # [doc = " Zstandard."] # [doc = ""] # [doc = " Used for ELF compression."] Zstandard , }
    };
}

CompressionFormat!()