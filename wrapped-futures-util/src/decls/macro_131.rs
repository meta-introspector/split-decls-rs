macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method."] TryFlattenStream < Fut > (try_flatten :: TryFlatten < Fut , Fut :: Ok >) : Debug + Sink + Stream + FusedStream + New [| x : Fut | try_flatten :: TryFlatten :: new (x)] where Fut : TryFuture) ;
    };
}

macro_131!();