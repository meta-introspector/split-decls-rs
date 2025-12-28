macro_rules! deps {
    () => {
        Limb!();
        Loss!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Loss { # [doc = " Combine the effect of two lost fractions."] # [inline] fn combine (self , less_significant : Loss) -> Loss { let mut more_significant = self ; if less_significant != Loss :: ExactlyZero { if more_significant == Loss :: ExactlyZero { more_significant = Loss :: LessThanHalf ; } else if more_significant == Loss :: ExactlyHalf { more_significant = Loss :: MoreThanHalf ; } } more_significant } # [doc = " Return the fraction lost were a bignum truncated losing the least"] # [doc = " significant `bits` bits."] # [inline] fn through_truncation (limbs : & [Limb] , bits : usize) -> Loss { if bits == 0 { return Loss :: ExactlyZero ; } let half_bit = bits - 1 ; let half_limb = half_bit / LIMB_BITS ; let (half_limb , rest) = if half_limb < limbs . len () { (limbs [half_limb] , & limbs [.. half_limb]) } else { (0 , limbs) } ; let half = 1 << (half_bit % LIMB_BITS) ; let has_half = half_limb & half != 0 ; let has_rest = half_limb & (half - 1) != 0 || ! sig :: is_all_zeros (rest) ; match (has_half , has_rest) { (false , false) => Loss :: ExactlyZero , (false , true) => Loss :: LessThanHalf , (true , false) => Loss :: ExactlyHalf , (true , true) => Loss :: MoreThanHalf , } } }
    };
}

impl_48!();