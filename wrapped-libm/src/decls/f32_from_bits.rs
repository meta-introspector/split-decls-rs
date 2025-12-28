macro_rules! f32_from_bits {
    () => {
        # [doc = " `f32::from_bits`"] # [allow (unnecessary_transmutes)] pub const fn f32_from_bits (bits : u32) -> f32 { unsafe { mem :: transmute :: < u32 , f32 > (bits) } }
    };
}

f32_from_bits!()