macro_rules! deps {
    () => {
        MultiProductInner!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < I > std :: fmt :: Debug for MultiProductInner < I > where I : Iterator + Clone + std :: fmt :: Debug , I :: Item : Clone + std :: fmt :: Debug , { debug_fmt_fields ! (MultiProductInner , iters , cur) ; }
    };
}

impl_63!()