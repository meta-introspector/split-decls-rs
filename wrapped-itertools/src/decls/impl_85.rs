macro_rules! deps {
    () => {
        Product!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < I , J > FusedIterator for Product < I , J > where I : FusedIterator , J : Clone + FusedIterator , I :: Item : Clone , { }
    };
}

impl_85!()