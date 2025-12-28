macro_rules! deps {
    () => {
        RefCnt!();
        Strategy!();
        ArcSwapAny!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < T , S : Strategy < T > > Display for ArcSwapAny < T , S > where T : Display + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . load () . fmt (formatter) } }
    };
}

impl_151!();