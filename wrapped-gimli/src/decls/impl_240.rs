macro_rules! deps {
    () => {
        RegisterRule!();
        ReaderOffset!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < T : ReaderOffset > RegisterRule < T > { fn is_defined (& self) -> bool { ! matches ! (* self , RegisterRule :: Undefined) } }
    };
}

impl_240!()