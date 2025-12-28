macro_rules! deps {
    () => {
        WeakKeyError!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl fmt :: Display for WeakKeyError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("WeakKey") } }
    };
}

impl_34!()