macro_rules! deps {
    () => {
        Ones!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < 'a > Ones < 'a > { # [inline] pub fn last_positive_bit_and_unset (n : & mut usize) -> usize { let last_bit = * n & n . wrapping_neg () ; let position = last_bit . trailing_zeros () ; * n &= * n - 1 ; position as usize } # [inline] fn first_positive_bit_and_unset (n : & mut usize) -> usize { let bit_idx = n . leading_zeros () ; let mask = ! ((1_usize) << (BITS as u32 - bit_idx - 1)) ; n . bitand_assign (mask) ; bit_idx as usize } }
    };
}

impl_113!()