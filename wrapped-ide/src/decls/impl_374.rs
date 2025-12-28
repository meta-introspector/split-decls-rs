macro_rules! deps {
    () => {
        TestId!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl fmt :: Display for TestId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TestId :: Name (name) => name . fmt (f) , TestId :: Path (path) => path . fmt (f) , } } }
    };
}

impl_374!()