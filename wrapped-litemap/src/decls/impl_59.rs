macro_rules! deps {
    () => {
        LiteMapVisitor!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < K , V , R > LiteMapVisitor < K , V , R > { fn new () -> Self { Self { marker : PhantomData , } } }
    };
}

impl_59!();