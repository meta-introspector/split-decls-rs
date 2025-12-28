macro_rules! Rle {
    () => {
        # [doc = " Status of RLE encoding of huffman code lengths."] struct Rle { pub z_count : u32 , pub repeat_count : u16 , pub prev_code_size : u8 , }
    };
}

Rle!()