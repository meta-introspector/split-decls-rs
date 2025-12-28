macro_rules! macro_133 {
    () => {
        delegate_all ! (# [doc = " Future for the [`and_then`](TryFutureExt::and_then) method."] AndThen < Fut1 , Fut2 , F > (TryFlatten < MapOk < Fut1 , F >, Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 , f : F | TryFlatten :: new (MapOk :: new (x , f))]) ;
    };
}

macro_133!()