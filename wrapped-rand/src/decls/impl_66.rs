macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        Rng!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , feature = "simd_support"))] impl Distribution < __m512i > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> __m512i { let mut buf = [0_u8 ; core :: mem :: size_of :: < __m512i > ()] ; rng . fill_bytes (& mut buf) ; unsafe { core :: mem :: transmute (buf) } } }
    };
}

impl_66!()