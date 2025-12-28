macro_rules! deps {
    () => {
        Uniform!();
        Distribution!();
        Rng!();
        StandardUniform!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Distribution < char > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> char { const GAP_SIZE : u32 = 0xDFFF - 0xD800 + 1 ; let range = Uniform :: new (GAP_SIZE , 0x11_0000) . unwrap () ; let mut n = range . sample (rng) ; if n <= 0xDFFF { n -= GAP_SIZE ; } unsafe { char :: from_u32_unchecked (n) } } }
    };
}

impl_73!();