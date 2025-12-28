macro_rules! deps {
    () => {
        UnwrapOrElseFn!();
    };
}

macro_rules! macro_142 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method."] UnwrapOrElse < Fut , F > (Map < IntoFuture < Fut >, UnwrapOrElseFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , unwrap_or_else_fn (f))]) ;
    };
}

macro_142!()