macro_rules! deps {
    () => {
        Odd!();
        Bounded!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < T > Odd < T > where T : Bounded + ? Sized , { # [doc = " Total size of the represented integer in bits."] pub const BITS : u32 = T :: BITS ; # [doc = " Total size of the represented integer in bytes."] pub const BYTES : usize = T :: BYTES ; }
    };
}

impl_225!()