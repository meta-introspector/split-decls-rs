macro_rules! deps {
    () => {
        PrinterBuilder!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Default for PrinterBuilder { fn default () -> PrinterBuilder { PrinterBuilder :: new () } }
    };
}

impl_24!()