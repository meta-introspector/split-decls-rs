macro_rules! deps {
    () => {
        IntoFn!();
    };
}

macro_rules! macro_135 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`err_into`](TryFutureExt::err_into) method."] ErrInto < Fut , E > (MapErr < Fut , IntoFn < E >>) : Debug + Future + FusedFuture + New [| x : Fut | MapErr :: new (x , into_fn ())]) ;
    };
}

macro_135!()