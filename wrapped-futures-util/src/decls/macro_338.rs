macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_338 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`flatten`](StreamExt::flatten) method."] Flatten < St > (flatten :: Flatten < St , St :: Item >) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St | flatten :: Flatten :: new (x)] where St : Stream) ;
    };
}

macro_338!();