macro_rules! deps {
    () => {
        ReaderOffset!();
        RegisterRule!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < T : ReaderOffset > RegisterRule < T > { fn is_defined (& self) -> bool { ! matches ! (* self , RegisterRule :: Undefined) } }
    };
}

impl_240!();