macro_rules! deps {
    () => {
        InspectOkFn!();
    };
}

macro_rules! macro_137 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method."] InspectOk < Fut , F > (Inspect < IntoFuture < Fut >, InspectOkFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Inspect :: new (IntoFuture :: new (x) , inspect_ok_fn (f))]) ;
    };
}

macro_137!();