macro_rules! deps {
    () => {
        InspectFn!();
        Sink!();
    };
}

macro_rules! macro_380 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`inspect`](StreamExt::inspect) method."] Inspect < St , F > (map :: Map < St , InspectFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St , f : F | map :: Map :: new (x , inspect_fn (f))]) ;
    };
}

macro_380!()