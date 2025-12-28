macro_rules! deps {
    () => {
        ArgPrinter!();
        DebugPrint!();
        ViaDebug!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , T : Debug > ViaDebug < T > for & ArgPrinter < 'a , T > { fn debug_string (& self) -> DebugPrint < 'a , T > { DebugPrint (self . 0) } }
    };
}

impl_16!();