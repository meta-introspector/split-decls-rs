macro_rules! f64_from_bits {
    () => {
        # [doc = " `f64::from_bits`"] # [allow (unnecessary_transmutes)] pub const fn f64_from_bits (bits : u64) -> f64 { unsafe { mem :: transmute :: < u64 , f64 > (bits) } }
    };
}

f64_from_bits!();