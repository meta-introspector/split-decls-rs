macro_rules! OUT_BUF_SIZE {
    () => {
        # [doc = " Size of the output buffer."] pub const OUT_BUF_SIZE : usize = (LZ_CODE_BUF_SIZE * 13) / 10 ;
    };
}

OUT_BUF_SIZE!()