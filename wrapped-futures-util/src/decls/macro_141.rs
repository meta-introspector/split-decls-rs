macro_rules! deps {
    () => {
        MapOkOrElseFn!();
    };
}

macro_rules! macro_141 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method."] MapOkOrElse < Fut , F , G > (Map < IntoFuture < Fut >, MapOkOrElseFn < F , G >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F , g : G | Map :: new (IntoFuture :: new (x) , map_ok_or_else_fn (f , g))]) ;
    };
}

macro_141!()