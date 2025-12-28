macro_rules! OkFn {
    () => {
        pub struct OkFn < E > (PhantomData < fn (E) >) ;
    };
}

OkFn!();