macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Shake256!();
        FieldElement!();
        RingElement!();
    };
}

macro_rules! sample_poly_cbd {
    () => {
        deps!();
        # [doc = " FIPS-203, Algorithm 8."] # [doc = ""] # [doc = " This is combined with PRF_{eta n}."] # [doc = ""] # [doc = " See section (4.2) in FIPS 203 on the instantiation of B^{65*eta} <=> PRF"] pub fn sample_poly_cbd (seed : & [u8] , b : u8 , prf_out : & mut [u8] , bits : & mut [u8] , eta : usize ,) -> Result < RingElement , UnknownCryptoError > { debug_assert_eq ! (seed . len () , 32) ; debug_assert ! (eta == 2 || eta == 3) ; let mut prf = shake256 :: Shake256 :: new () ; prf . absorb (seed) ? ; prf . absorb (& [b]) ? ; prf . squeeze (prf_out) ? ; bytes_to_bits (prf_out , bits) ; let mut f : RingElement = RingElement :: zero () ; for i in 0 .. 256 { let mut x : u8 = 0 ; let mut y : u8 = 0 ; for j in 0 .. eta { x += bits [(2 * i * eta) + j] ; y += bits [(2 * i * eta) + eta + j] ; } debug_assert ! (x <= eta as u8) ; debug_assert ! (y <= eta as u8) ; f [i] = FieldElement :: new (x as u32) - FieldElement :: new (y as u32) ; debug_assert ! ((f [i] . 0 <= eta as u32) || (KYBER_Q - (eta as u32) <= f [i] . 0 && f [i] . 0 < KYBER_Q)) ; } Ok (f) }
    };
}

sample_poly_cbd!()