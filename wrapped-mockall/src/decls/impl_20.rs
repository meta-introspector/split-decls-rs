macro_rules! deps {
    () => {
        ViaNothing!();
        ArgPrinter!();
        NothingPrint!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > ViaNothing for ArgPrinter < '_ , T > { fn debug_string (& self) -> NothingPrint { NothingPrint } }
    };
}

impl_20!();