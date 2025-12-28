macro_rules! deps {
    () => {
        DFloat!();
    };
}

macro_rules! impl_d_float {
    () => {
        deps!();
        macro_rules ! impl_d_float { ($ ($ X : ident $ D : ident) ,*) => { $ (impl DFloat for $ D { type H = $ X ; fn narrow (self) -> Self :: H { self as $ X } }) * } ; }
    };
}

impl_d_float!();