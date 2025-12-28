macro_rules! deps {
    () => {
        LiteralCoder!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl LiteralCoder { pub fn new (lc : u32 , lp : u32) -> Self { Self { lc , literal_pos_mask : (1 << lp) - 1 , } } pub (crate) fn get_sub_coder_index (& self , prev_byte : u32 , pos : u32) -> u32 { let low = prev_byte >> (8 - self . lc) ; let high = (pos & self . literal_pos_mask) << self . lc ; low + high } }
    };
}

impl_312!();