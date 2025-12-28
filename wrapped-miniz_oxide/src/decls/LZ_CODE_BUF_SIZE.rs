macro_rules! LZ_CODE_BUF_SIZE {
    () => {
        # [doc = " Size of the buffer of lz77 encoded data."] pub const LZ_CODE_BUF_SIZE : usize = 64 * 1024 ;
    };
}

LZ_CODE_BUF_SIZE!()