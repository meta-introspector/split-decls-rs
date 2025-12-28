macro_rules! deps {
    () => {
        NonZero!();
        Constants!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < T > NonZero < T > where T : Constants , { # [doc = " The value `1`."] pub const ONE : Self = Self (T :: ONE) ; # [doc = " Maximum value this integer can express."] pub const MAX : Self = Self (T :: MAX) ; }
    };
}

impl_182!();