macro_rules! LineRow {
    () => {
        struct LineRow { address : u64 , file_index : u64 , line : u32 , column : u32 , }
    };
}

LineRow!();