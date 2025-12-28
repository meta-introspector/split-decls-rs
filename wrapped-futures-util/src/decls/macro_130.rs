macro_rules! macro_130 {
    () => {
        delegate_all ! (# [doc = " Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method."] TryFlattenErr < Fut1 , Fut2 > (try_flatten_err :: TryFlattenErr < Fut1 , Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 | try_flatten_err :: TryFlattenErr :: new (x)]) ;
    };
}

macro_130!()