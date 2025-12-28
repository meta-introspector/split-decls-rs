macro_rules! BIGINT_LIMBS {
    () => {
        # [doc = " The number of limbs for the bigint."] pub const BIGINT_LIMBS : usize = BIGINT_BITS / LIMB_BITS ;
    };
}

BIGINT_LIMBS!();