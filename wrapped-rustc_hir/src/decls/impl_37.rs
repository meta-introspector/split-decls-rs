macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : PrintAttribute > PrintAttribute for & T { fn should_render (& self) -> bool { T :: should_render (self) } fn print_attribute (& self , p : & mut Printer) { T :: print_attribute (self , p) } }
    };
}

impl_37!()