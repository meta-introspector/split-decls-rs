macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl PrintAttribute for u128 { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (self . to_string ()) } }
    };
}

impl_36!()