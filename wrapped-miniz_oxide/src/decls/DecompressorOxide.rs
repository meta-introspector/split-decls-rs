macro_rules! deps {
    () => {
        State!();
        BitBuffer!();
        BigArray!();
        HuffmanTable!();
    };
}

macro_rules! DecompressorOxide {
    () => {
        deps!();
        # [doc = " Main decompression struct."] # [doc = ""] # [cfg_attr (not (feature = "rustc-dep-of-std") , derive (Clone))] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct DecompressorOxide { # [doc = " Current state of the decompressor."] state : core :: State , # [doc = " Number of bits in the bit buffer."] num_bits : u32 , # [doc = " Zlib CMF"] z_header0 : u32 , # [doc = " Zlib FLG"] z_header1 : u32 , # [doc = " Adler32 checksum from the zlib header."] z_adler32 : u32 , # [doc = " 1 if the current block is the last block, 0 otherwise."] finish : u8 , # [doc = " The type of the current block."] # [doc = " or if in a dynamic block, which huffman table we are currently"] block_type : u8 , # [doc = " 1 if the adler32 value should be checked."] check_adler32 : u32 , # [doc = " Last match distance."] dist : u32 , # [doc = " Variable used for match length, symbols, and a number of other things."] counter : u32 , # [doc = " Number of extra bits for the last length or distance code."] num_extra : u8 , # [doc = " Number of entries in each huffman table."] table_sizes : [u16 ; MAX_HUFF_TABLES] , # [doc = " Buffer of input data."] bit_buf : BitBuffer , # [doc = " Huffman tables."] tables : [HuffmanTable ; MAX_HUFF_TABLES] , # [cfg_attr (feature = "serde" , serde (with = "BigArray"))] code_size_literal : [u8 ; MAX_HUFF_SYMBOLS_0] , code_size_dist : [u8 ; MAX_HUFF_SYMBOLS_1] , code_size_huffman : [u8 ; MAX_HUFF_SYMBOLS_2] , # [doc = " Raw block header."] raw_header : [u8 ; 4] , # [doc = " Huffman length codes."] # [cfg_attr (feature = "serde" , serde (with = "BigArray"))] len_codes : [u8 ; LEN_CODES_SIZE] , }
    };
}

DecompressorOxide!()