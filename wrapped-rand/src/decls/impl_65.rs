macro_rules! deps {
    () => {
        StandardUniform!();
        Distribution!();
        Rng!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] impl Distribution < __m256i > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> __m256i { let mut buf = [0_u8 ; core :: mem :: size_of :: < __m256i > ()] ; rng . fill_bytes (& mut buf) ; unsafe { core :: mem :: transmute (buf) } } }
    };
}

impl_65!()