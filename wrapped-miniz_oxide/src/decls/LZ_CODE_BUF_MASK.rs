macro_rules! LZ_CODE_BUF_MASK {
    () => {
        pub const LZ_CODE_BUF_MASK : usize = LZ_CODE_BUF_SIZE - 1 ;
    };
}

LZ_CODE_BUF_MASK!();