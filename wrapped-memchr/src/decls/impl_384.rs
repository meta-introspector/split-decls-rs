macro_rules! deps {
    () => {
        SensibleMoveMask!();
        MoveMask!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl MoveMask for SensibleMoveMask { # [inline (always)] fn all_zeros_except_least_significant (n : usize) -> SensibleMoveMask { debug_assert ! (n < 32) ; SensibleMoveMask (! ((1 << n) - 1)) } # [inline (always)] fn has_non_zero (self) -> bool { self . 0 != 0 } # [inline (always)] fn count_ones (self) -> usize { self . 0 . count_ones () as usize } # [inline (always)] fn and (self , other : SensibleMoveMask) -> SensibleMoveMask { SensibleMoveMask (self . 0 & other . 0) } # [inline (always)] fn or (self , other : SensibleMoveMask) -> SensibleMoveMask { SensibleMoveMask (self . 0 | other . 0) } # [inline (always)] fn clear_least_significant_bit (self) -> SensibleMoveMask { SensibleMoveMask (self . 0 & (self . 0 - 1)) } # [inline (always)] fn first_offset (self) -> usize { self . get_for_offset () . trailing_zeros () as usize } # [inline (always)] fn last_offset (self) -> usize { 32 - self . get_for_offset () . leading_zeros () as usize - 1 } }
    };
}

impl_384!()