macro_rules! deps {
    () => {
        VecType!();
    };
}

macro_rules! shl {
    () => {
        deps!();
        # [doc = " Shift-left buffer by n bits."] # [inline] pub fn shl (x : & mut VecType , n : usize) -> Option < () > { let rem = n % LIMB_BITS ; let div = n / LIMB_BITS ; if rem != 0 { shl_bits (x , rem) ? ; } if div != 0 { shl_limbs (x , div) ? ; } Some (()) }
    };
}

shl!()