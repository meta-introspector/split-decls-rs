macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! print_skip {
    () => {
        deps!();
        macro_rules ! print_skip { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { false } fn print_attribute (& self , _ : & mut Printer) { } }) * } ; }
    };
}

print_skip!();