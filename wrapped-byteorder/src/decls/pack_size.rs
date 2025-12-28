macro_rules! pack_size {
    () => {
        # [inline] fn pack_size (n : u64) -> usize { (8 - ((n | 1) . leading_zeros () >> 3)) as usize }
    };
}

pack_size!()