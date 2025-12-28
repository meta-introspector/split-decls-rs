macro_rules! f64_to_bits {
    () => {
        # [doc = " `f64::to_bits`"] # [allow (dead_code)] # [allow (unnecessary_transmutes)] pub const fn f64_to_bits (x : f64) -> u64 { unsafe { mem :: transmute :: < f64 , u64 > (x) } }
    };
}

f64_to_bits!()