macro_rules! deps {
    () => {
        UpgradeExpected!();
        Result!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl fmt :: Display for UpgradeExpected { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("upgrade expected but not completed") } }
    };
}

impl_223!()