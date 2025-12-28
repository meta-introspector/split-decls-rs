macro_rules! deps {
    () => {
        RefCnt!();
        Strategy!();
        Guard!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < T : Debug + RefCnt , S : Strategy < T > > Debug for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
    };
}

impl_144!()