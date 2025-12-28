macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : PrintAttribute > PrintAttribute for ThinVec < T > { fn should_render (& self) -> bool { self . is_empty () || self [0] . should_render () } fn print_attribute (& self , p : & mut Printer) { let mut last_printed = false ; p . word ("[") ; for i in self { if last_printed { p . word_space (",") ; } i . print_attribute (p) ; last_printed = i . should_render () ; } p . word ("]") ; } }
    };
}

impl_39!()