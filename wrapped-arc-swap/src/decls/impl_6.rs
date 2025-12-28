macro_rules! deps {
    () => {
        Strategy!();
        RefCnt!();
        ArcSwapAny!();
        Guard!();
        Access!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > Access < T > for ArcSwapAny < T , S > { type Guard = Guard < T , S > ; fn load (& self) -> Self :: Guard { self . load () } }
    };
}

impl_6!();