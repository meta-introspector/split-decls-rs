macro_rules! macro_60 {
    () => {
        delegate_all ! (# [doc = " Stream for the [`into_stream`](FutureExt::into_stream) method."] IntoStream < F > (crate :: stream :: Once < F >) : Debug + Stream + FusedStream + New [| x : F | crate :: stream :: Once :: new (x)]) ;
    };
}

macro_60!()