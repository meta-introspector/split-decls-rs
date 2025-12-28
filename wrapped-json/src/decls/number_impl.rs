macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! number_impl {
    () => {
        deps!();
        macro_rules ! number_impl { ($ ($ ty : ident) *) => { $ (impl Number for $ ty { }) * } ; }
    };
}

number_impl!()