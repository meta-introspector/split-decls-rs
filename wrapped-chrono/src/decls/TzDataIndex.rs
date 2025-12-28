macro_rules! TzDataIndex {
    () => {
        # [doc = " Index entry of the `tzdata` file."] struct TzDataIndex { name : Box < [u8] > , offset : u32 , length : u32 , }
    };
}

TzDataIndex!();