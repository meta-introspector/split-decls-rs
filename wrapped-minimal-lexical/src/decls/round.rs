macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! round {
    () => {
        deps!();
        # [doc = " Round an extended-precision float to the nearest machine float."] # [doc = ""] # [doc = " Shifts the significant digits into place, adjusts the exponent,"] # [doc = " so it can be easily converted to a native float."] # [cfg_attr (not (feature = "compact") , inline)] pub fn round < F , Cb > (fp : & mut ExtendedFloat , cb : Cb) where F : Float , Cb : Fn (& mut ExtendedFloat , i32) , { let fp_inf = ExtendedFloat { mant : 0 , exp : F :: INFINITE_POWER , } ; let mantissa_shift = 64 - F :: MANTISSA_SIZE - 1 ; if - fp . exp >= mantissa_shift { let shift = - fp . exp + 1 ; debug_assert ! (shift <= 65) ; cb (fp , shift . min (64)) ; fp . exp = (fp . mant >= F :: HIDDEN_BIT_MASK) as i32 ; return ; } cb (fp , mantissa_shift) ; let carry_mask = F :: CARRY_MASK ; if fp . mant & carry_mask == carry_mask { fp . mant >>= 1 ; fp . exp += 1 ; } if fp . exp >= F :: INFINITE_POWER { * fp = fp_inf ; return ; } fp . mant &= F :: MANTISSA_MASK ; }
    };
}

round!();