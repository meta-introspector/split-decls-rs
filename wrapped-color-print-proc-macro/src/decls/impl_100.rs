macro_rules! deps {
    () => {
        Specified!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Specified { # [inline] fn is_true (& self) -> bool { matches ! (self , Specified :: True) } }
    };
}

impl_100!();