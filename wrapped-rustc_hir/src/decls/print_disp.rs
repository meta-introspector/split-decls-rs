macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! print_disp {
    () => {
        deps!();
        macro_rules ! print_disp { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (format ! ("{}" , self)) ; } }) * } ; }
    };
}

print_disp!();