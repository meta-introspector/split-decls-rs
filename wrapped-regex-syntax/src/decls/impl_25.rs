macro_rules! deps {
    () => {
        Printer!();
        PrinterBuilder!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl PrinterBuilder { fn new () -> PrinterBuilder { PrinterBuilder { _priv : () } } fn build (& self) -> Printer { Printer { _priv : () } } }
    };
}

impl_25!();