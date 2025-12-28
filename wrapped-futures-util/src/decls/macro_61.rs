macro_rules! deps {
    () => {
        IntoFn!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`map_into`](FutureExt::map_into) combinator."] MapInto < Fut , T > (Map < Fut , IntoFn < T >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , into_fn ())]) ;
    };
}

macro_61!();