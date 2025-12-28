macro_rules! IntoFn {
    () => {
        pub struct IntoFn < T > (PhantomData < fn () -> T >) ;
    };
}

IntoFn!()