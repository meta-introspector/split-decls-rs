macro_rules! deps {
    () => {
        FieldElement!();
        RingElementNTT!();
        Shake128!();
        UnknownCryptoError!();
    };
}

macro_rules! sample_ntt {
    () => {
        deps!();
        # [doc = " FIPS-203, Algorithm 7."] pub fn sample_ntt (seed : & [u8 ; 32] , ij : & [u8 ; 2]) -> Result < RingElementNTT , UnknownCryptoError > { let mut xof = shake128 :: Shake128 :: new () ; xof . absorb (seed) ? ; xof . absorb (ij) ? ; let mut a_hat = RingElementNTT :: zero () ; let mut j = 0 ; while j < 256 { let mut c = [0u8 ; 3] ; xof . squeeze (& mut c) ? ; let d1 : i16 = (c [0] as i16) + 256 * ((c [1] as i16) & 15) ; debug_assert ! (d1 >= 0 || d1 < 2i16 . pow (12)) ; let d2 : i16 = ((c [1] as i16) >> 4u16) + 16i16 * (c [2] as i16) ; debug_assert ! (d2 >= 0 || d2 < 2i16 . pow (12)) ; if d1 < KYBER_Q as i16 { a_hat [j] = FieldElement (d1 as u32) ; j += 1 ; } if d2 < KYBER_Q as i16 && j < 256 { a_hat [j] = FieldElement (d2 as u32) ; j += 1 ; } } Ok (a_hat) }
    };
}

sample_ntt!()