macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! impl_931 {
    () => {
        deps!();
        impl fmt :: Debug for TestDB { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TestDB") . finish () } }
    };
}

impl_931!()