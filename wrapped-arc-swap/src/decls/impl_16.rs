macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : Debug + RefCnt , S : Strategy < T > > Debug for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
    };
}

impl_16!()