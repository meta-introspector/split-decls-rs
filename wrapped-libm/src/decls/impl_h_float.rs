macro_rules! deps {
    () => {
        HFloat!();
    };
}

macro_rules! impl_h_float {
    () => {
        deps!();
        macro_rules ! impl_h_float { ($ ($ H : ident $ X : ident) ,*) => { $ (impl HFloat for $ H { type D = $ X ; fn widen (self) -> Self :: D { self as $ X } }) * } ; }
    };
}

impl_h_float!();