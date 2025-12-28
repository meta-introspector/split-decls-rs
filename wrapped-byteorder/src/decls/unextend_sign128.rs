macro_rules! unextend_sign128 {
    () => {
        # [inline] fn unextend_sign128 (val : i128 , nbytes : usize) -> u128 { let shift = (16 - nbytes) * 8 ; (val << shift) as u128 >> shift }
    };
}

unextend_sign128!();