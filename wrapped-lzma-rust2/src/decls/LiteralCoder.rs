macro_rules! LiteralCoder {
    () => {
        pub (crate) struct LiteralCoder { lc : u32 , literal_pos_mask : u32 , }
    };
}

LiteralCoder!();