macro_rules! deps {
    () => {
        FnContext!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl FnContext { # [inline] fn new (migrated : bool) -> Self { FnContext { migrated , _marker : PhantomData , } } }
    };
}

impl_330!();