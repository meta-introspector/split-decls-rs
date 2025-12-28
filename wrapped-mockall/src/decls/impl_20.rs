macro_rules! deps {
    () => {
        ArgPrinter!();
        ViaNothing!();
        NothingPrint!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > ViaNothing for ArgPrinter < '_ , T > { fn debug_string (& self) -> NothingPrint { NothingPrint } }
    };
}

impl_20!()