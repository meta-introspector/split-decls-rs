macro_rules! deps {
    () => {
        NonZero!();
        Bounded!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T > NonZero < T > where T : Bounded + ? Sized , { # [doc = " Total size of the represented integer in bits."] pub const BITS : u32 = T :: BITS ; # [doc = " Total size of the represented integer in bytes."] pub const BYTES : usize = T :: BYTES ; }
    };
}

impl_181!()