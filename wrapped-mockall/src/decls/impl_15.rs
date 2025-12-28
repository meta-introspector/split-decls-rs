macro_rules! deps {
    () => {
        NothingPrint!();
        ArgPrinter!();
        ViaNothing!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > ViaNothing for ArgPrinter < '_ , T > { fn debug_string (& self) -> NothingPrint { NothingPrint } }
    };
}

impl_15!()