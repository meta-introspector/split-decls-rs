macro_rules! macro_129 {
    () => {
        delegate_all ! (# [doc = " Future for the [`try_flatten`](TryFutureExt::try_flatten) method."] TryFlatten < Fut1 , Fut2 > (try_flatten :: TryFlatten < Fut1 , Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 | try_flatten :: TryFlatten :: new (x)]) ;
    };
}

macro_129!();