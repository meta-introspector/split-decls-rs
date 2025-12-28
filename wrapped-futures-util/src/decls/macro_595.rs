macro_rules! deps {
    () => {
        InspectErrFn!();
        Sink!();
    };
}

macro_rules! macro_595 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method."] InspectErr < St , F > (Inspect < IntoStream < St >, InspectErrFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Inspect :: new (IntoStream :: new (x) , inspect_err_fn (f))]) ;
    };
}

macro_595!()