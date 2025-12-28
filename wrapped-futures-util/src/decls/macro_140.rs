macro_rules! deps {
    () => {
        MapErrFn!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`map_err`](TryFutureExt::map_err) method."] MapErr < Fut , F > (Map < IntoFuture < Fut >, MapErrFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , map_err_fn (f))]) ;
    };
}

macro_140!();