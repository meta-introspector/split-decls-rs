macro_rules! FCHECK_DIVISOR {
    () => {
        # [doc = " The 16-bit value consisting of CMF and FLG must be divisible by this to be valid."] const FCHECK_DIVISOR : u8 = 31 ;
    };
}

FCHECK_DIVISOR!()