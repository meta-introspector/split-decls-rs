macro_rules! deps {
    () => {
        DecompressorOxide!();
    };
}

macro_rules! BlockBoundaryState {
    () => {
        deps!();
        # [doc = " Minimal data representing the [`DecompressorOxide`] state when it is between deflate blocks"] # [doc = " (i.e. [`decompress()`] has returned [`TINFLStatus::BlockBoundary`])."] # [doc = " This can be serialized along with the last 32KiB of the output buffer, then passed to"] # [doc = " [`DecompressorOxide::from_block_boundary_state()`] to resume decompression from the same point."] # [doc = ""] # [doc = " The Zlib/Adler32 fields can be ignored if you aren't using those features"] # [doc = " ([`TINFL_FLAG_PARSE_ZLIB_HEADER`], [`TINFL_FLAG_COMPUTE_ADLER32`])."] # [doc = " When deserializing, you can reconstruct `bit_buf` from the previous byte in the input file"] # [doc = " (if you still have access to it), so `num_bits` is the only field that is always required."] # [derive (Clone)] # [cfg (feature = "block-boundary")] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct BlockBoundaryState { # [doc = " The number of bits from the last byte of input consumed,"] # [doc = " that are needed for decoding the next deflate block."] # [doc = " Value is in range `0..=7`"] pub num_bits : u8 , # [doc = " The `num_bits` MSBs from the last byte of input consumed,"] # [doc = " that are needed for decoding the next deflate block."] # [doc = " Stored in the LSBs of this field."] pub bit_buf : u8 , # [doc = " Zlib CMF"] pub z_header0 : u32 , # [doc = " Zlib FLG"] pub z_header1 : u32 , # [doc = " Adler32 checksum of the data decompressed so far"] pub check_adler32 : u32 , }
    };
}

BlockBoundaryState!()