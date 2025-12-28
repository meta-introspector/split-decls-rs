macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < T : Debug + RefCnt , S : Strategy < T > > Debug for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
    };
}

impl_144!();