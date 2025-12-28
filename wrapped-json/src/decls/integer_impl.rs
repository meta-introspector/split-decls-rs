macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! integer_impl {
    () => {
        deps!();
        macro_rules ! integer_impl { ($ ($ ty : tt) *) => { $ (impl Integer for $ ty { const ZERO : Self = 0 ; }) * } ; }
    };
}

integer_impl!()