macro_rules! extend_sign {
    () => {
        # [inline] fn extend_sign (val : u64 , nbytes : usize) -> i64 { let shift = (8 - nbytes) * 8 ; (val << shift) as i64 >> shift }
    };
}

extend_sign!();