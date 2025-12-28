macro_rules! deps {
    () => {
        DirectDeref!();
        ArcSwapAny!();
        Strategy!();
        Access!();
        Guard!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T , S : Strategy < Rc < T > > > Access < T > for ArcSwapAny < Rc < T > , S > { type Guard = DirectDeref < Rc < T > , S > ; fn load (& self) -> Self :: Guard { DirectDeref (self . load ()) } }
    };
}

impl_11!()