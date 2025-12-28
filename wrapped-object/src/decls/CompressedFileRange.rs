macro_rules! deps {
    () => {
        CompressionFormat!();
    };
}

macro_rules! CompressedFileRange {
    () => {
        deps!();
        # [doc = " A range in a file that may be compressed."] # [doc = ""] # [doc = " Returned by [`ObjectSection::compressed_file_range`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CompressedFileRange { # [doc = " The data compression format."] pub format : CompressionFormat , # [doc = " The file offset of the compressed data."] pub offset : u64 , # [doc = " The compressed data size."] pub compressed_size : u64 , # [doc = " The uncompressed data size."] pub uncompressed_size : u64 , }
    };
}

CompressedFileRange!();