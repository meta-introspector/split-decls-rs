macro_rules! deps {
    () => {
        InspectErrFn!();
    };
}

macro_rules! macro_138 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method."] InspectErr < Fut , F > (Inspect < IntoFuture < Fut >, InspectErrFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Inspect :: new (IntoFuture :: new (x) , inspect_err_fn (f))]) ;
    };
}

macro_138!();