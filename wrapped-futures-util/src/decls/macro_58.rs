macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_58 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`flatten_stream`](FutureExt::flatten_stream) method."] FlattenStream < F > (flatten :: Flatten < F , < F as Future >:: Output >) : Debug + Sink + Stream + FusedStream + New [| x : F | flatten :: Flatten :: new (x)] where F : Future) ;
    };
}

macro_58!();