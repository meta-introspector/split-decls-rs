macro_rules! deps {
    () => {
        FieldElement!();
        RingElementNTT!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl Mul for RingElementNTT { type Output = Self ; # [doc = " FIPS-203, Algorithm 11."] # [doc = ""] # [doc = " self  = f_hat"] # [doc = " other = g_hat"] fn mul (self , other : Self) -> Self { let mut h_hat = Self :: zero () ; for i in 0 .. 128 { let a0 = self . coefficients [2 * i] ; let a1 = self . coefficients [2 * i + 1] ; let b0 = other . coefficients [2 * i] ; let b1 = other . coefficients [2 * i + 1] ; let (c0 , c1) = Self :: base_case_multiply (a0 , a1 , b0 , b1 , FieldElement (GAMMA_ALL [i])) ; h_hat [2 * i] = c0 ; h_hat [2 * i + 1] = c1 ; } h_hat } }
    };
}

impl_439!()