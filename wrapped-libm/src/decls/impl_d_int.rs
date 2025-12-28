macro_rules! deps {
    () => {
        DInt!();
        MinInt!();
    };
}

macro_rules! impl_d_int {
    () => {
        deps!();
        macro_rules ! impl_d_int { ($ ($ X : ident $ D : ident) ,*) => { $ (impl DInt for $ D { type H = $ X ; fn lo (self) -> Self :: H { self as $ X } fn hi (self) -> Self :: H { (self >> <$ X as MinInt >:: BITS) as $ X } }) * } ; }
    };
}

impl_d_int!()