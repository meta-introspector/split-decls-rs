macro_rules! deps {
    () => {
        MapOkFn!();
    };
}

macro_rules! macro_139 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`map_ok`](TryFutureExt::map_ok) method."] MapOk < Fut , F > (Map < IntoFuture < Fut >, MapOkFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , map_ok_fn (f))]) ;
    };
}

macro_139!()