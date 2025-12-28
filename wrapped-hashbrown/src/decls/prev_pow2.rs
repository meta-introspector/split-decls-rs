macro_rules! prev_pow2 {
    () => {
        # [doc = " Find the previous power of 2. If it's already a power of 2, it's unchanged."] # [doc = " Passing zero is undefined behavior."] pub (crate) fn prev_pow2 (z : usize) -> usize { let shift = mem :: size_of :: < usize > () * 8 - 1 ; 1 << (shift - (z . leading_zeros () as usize)) }
    };
}

prev_pow2!();