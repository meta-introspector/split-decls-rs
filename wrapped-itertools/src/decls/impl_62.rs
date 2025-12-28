macro_rules! deps {
    () => {
        MultiProduct!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < I > std :: fmt :: Debug for MultiProduct < I > where I : Iterator + Clone + std :: fmt :: Debug , I :: Item : Clone + std :: fmt :: Debug , { debug_fmt_fields ! (MultiProduct , 0) ; }
    };
}

impl_62!()