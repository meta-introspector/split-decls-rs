macro_rules! deps {
    () => {
        Access!();
        Guard!();
        DirectDeref!();
        Strategy!();
        ArcSwapAny!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T , S : Strategy < Arc < T > > > Access < T > for ArcSwapAny < Arc < T > , S > { type Guard = DirectDeref < Arc < T > , S > ; fn load (& self) -> Self :: Guard { DirectDeref (self . load ()) } }
    };
}

impl_9!()