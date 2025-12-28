macro_rules! f32_to_bits {
    () => {
        # [doc = " `f32::to_bits`"] # [allow (dead_code)] # [allow (unnecessary_transmutes)] pub const fn f32_to_bits (x : f32) -> u32 { unsafe { mem :: transmute :: < f32 , u32 > (x) } }
    };
}

f32_to_bits!()