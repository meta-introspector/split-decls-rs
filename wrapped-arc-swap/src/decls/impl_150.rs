macro_rules! deps {
    () => {
        ArcSwapAny!();
        Strategy!();
        RefCnt!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T , S : Strategy < T > > Debug for ArcSwapAny < T , S > where T : Debug + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { formatter . debug_tuple ("ArcSwapAny") . field (& self . load ()) . finish () } }
    };
}

impl_150!()