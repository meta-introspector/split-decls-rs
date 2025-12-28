macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! shr2_round {
    () => {
        deps!();
        # [doc = " Divide by 4, rounding with the given mode"] const fn shr2_round (mut x : u128 , round : Round) -> u128 { let t = (x as u32) & 0b111 ; x >>= 2 ; match round { Round :: Nearest => x + ((0b11001000_u8 >> t) & 1) as u128 , Round :: Negative => x , Round :: Zero => x , Round :: Positive => x + (t & 0b11 != 0) as u128 , } }
    };
}

shr2_round!()