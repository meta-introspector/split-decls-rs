macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T : Display + RefCnt , S : Strategy < T > > Display for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
    };
}

impl_145!()