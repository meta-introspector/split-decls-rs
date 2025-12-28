macro_rules! deps {
    () => {
        Sink!();
        IntoFn!();
    };
}

macro_rules! macro_593 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`err_into`](super::TryStreamExt::err_into) method."] ErrInto < St , E > (MapErr < St , IntoFn < E >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St | MapErr :: new (x , into_fn ())]) ;
    };
}

macro_593!()