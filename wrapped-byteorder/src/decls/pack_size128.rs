macro_rules! pack_size128 {
    () => {
        # [inline] fn pack_size128 (n : u128) -> usize { (16 - ((n | 1) . leading_zeros () >> 3)) as usize }
    };
}

pack_size128!();