macro_rules! EnumerateAndAdjust {
    () => {
        pub struct EnumerateAndAdjust < I > { enumerate : Enumerate < I > , gap_pos : usize , gap_len : usize , }
    };
}

EnumerateAndAdjust!()