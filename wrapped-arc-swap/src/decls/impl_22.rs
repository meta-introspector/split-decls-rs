macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T , S : Strategy < T > > Debug for ArcSwapAny < T , S > where T : Debug + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { formatter . debug_tuple ("ArcSwapAny") . field (& self . load ()) . finish () } }
    };
}

impl_22!()