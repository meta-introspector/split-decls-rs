macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T : PrintAttribute > PrintAttribute for Option < T > { fn should_render (& self) -> bool { self . as_ref () . is_some_and (| x | x . should_render ()) } fn print_attribute (& self , p : & mut Printer) { if let Some (i) = self { T :: print_attribute (i , p) } } }
    };
}

impl_38!();