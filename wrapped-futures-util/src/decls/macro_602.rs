macro_rules! deps {
    () => {
        MapOkFn!();
        Sink!();
    };
}

macro_rules! macro_602 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`map_ok`](super::TryStreamExt::map_ok) method."] MapOk < St , F > (Map < IntoStream < St >, MapOkFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Map :: new (IntoStream :: new (x) , map_ok_fn (f))]) ;
    };
}

macro_602!()