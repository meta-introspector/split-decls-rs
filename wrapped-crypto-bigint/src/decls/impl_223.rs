macro_rules! deps {
    () => {
        Integer!();
        Odd!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < T > Odd < T > { # [doc = " Create a new odd integer."] pub fn new (n : T) -> CtOption < Self > where T : Integer , { let is_odd = n . is_odd () ; CtOption :: new (Self (n) , is_odd) } # [doc = " Returns the inner value."] pub fn get (self) -> T { self . 0 } }
    };
}

impl_223!();