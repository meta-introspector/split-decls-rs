macro_rules! deps {
    () => {
        InspectOkFn!();
        Sink!();
    };
}

macro_rules! macro_594 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method."] InspectOk < St , F > (Inspect < IntoStream < St >, InspectOkFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Inspect :: new (IntoStream :: new (x) , inspect_ok_fn (f))]) ;
    };
}

macro_594!();