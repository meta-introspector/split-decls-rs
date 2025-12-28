macro_rules! deps {
    () => {
        IntoFn!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`ok_into`](TryFutureExt::ok_into) method."] OkInto < Fut , E > (MapOk < Fut , IntoFn < E >>) : Debug + Future + FusedFuture + New [| x : Fut | MapOk :: new (x , into_fn ())]) ;
    };
}

macro_136!();