macro_rules! deps {
    () => {
        PrinterBuilder!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl Default for PrinterBuilder { fn default () -> PrinterBuilder { PrinterBuilder :: new () } }
    };
}

impl_178!();