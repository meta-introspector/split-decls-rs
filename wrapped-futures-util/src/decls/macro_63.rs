macro_rules! deps {
    () => {
        InspectFn!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`inspect`](FutureExt::inspect) method."] Inspect < Fut , F > (map :: Map < Fut , InspectFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | map :: Map :: new (x , inspect_fn (f))]) ;
    };
}

macro_63!()