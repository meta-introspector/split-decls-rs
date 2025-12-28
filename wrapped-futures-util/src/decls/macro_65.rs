macro_rules! deps {
    () => {
        OkFn!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`unit_error`](super::FutureExt::unit_error) combinator."] UnitError < Fut > (Map < Fut , OkFn < () >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , ok_fn ())]) ;
    };
}

macro_65!();