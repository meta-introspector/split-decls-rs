macro_rules! deps {
    () => {
        Sink!();
        MapErrFn!();
    };
}

macro_rules! macro_603 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`map_err`](super::TryStreamExt::map_err) method."] MapErr < St , F > (Map < IntoStream < St >, MapErrFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Map :: new (IntoStream :: new (x) , map_err_fn (f))]) ;
    };
}

macro_603!();