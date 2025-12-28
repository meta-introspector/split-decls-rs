macro_rules! deps {
    () => {
        NonZero!();
        Zero!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < T > NonZero < T > { # [doc = " Create a new non-zero integer."] pub fn new (n : T) -> CtOption < Self > where T : Zero , { let is_zero = n . is_zero () ; CtOption :: new (Self (n) , ! is_zero) } # [doc = " Returns the inner value."] pub fn get (self) -> T { self . 0 } }
    };
}

impl_179!();