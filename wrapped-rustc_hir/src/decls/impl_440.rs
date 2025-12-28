macro_rules! deps {
    () => {
        EnumerateAndAdjust!();
        DotDotPos!();
        EnumerateAndAdjustIterator!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl < T : ExactSizeIterator > EnumerateAndAdjustIterator for T { fn enumerate_and_adjust (self , expected_len : usize , gap_pos : hir :: DotDotPos ,) -> EnumerateAndAdjust < Self > where Self : Sized , { let actual_len = self . len () ; EnumerateAndAdjust { enumerate : self . enumerate () , gap_pos : gap_pos . as_opt_usize () . unwrap_or (expected_len) , gap_len : expected_len - actual_len , } } }
    };
}

impl_440!()