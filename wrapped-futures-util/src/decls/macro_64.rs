macro_rules! deps {
    () => {
        OkFn!();
    };
}

macro_rules! macro_64 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`never_error`](super::FutureExt::never_error) combinator."] NeverError < Fut > (Map < Fut , OkFn < Infallible >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , ok_fn ())]) ;
    };
}

macro_64!()