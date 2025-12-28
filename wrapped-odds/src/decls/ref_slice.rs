macro_rules! ref_slice {
    () => {
        # [doc = " Create a length 1 slice out of a reference"] pub fn ref_slice < T > (ptr : & T) -> & [T] { unsafe { std :: slice :: from_raw_parts (ptr , 1) } }
    };
}

ref_slice!();