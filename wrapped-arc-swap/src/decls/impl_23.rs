macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T , S : Strategy < T > > Display for ArcSwapAny < T , S > where T : Display + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . load () . fmt (formatter) } }
    };
}

impl_23!()