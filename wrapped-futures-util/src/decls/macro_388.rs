macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_388 {
    () => {
        deps!();
        delegate_all ! (# [doc = " Stream for the [`flat_map`](StreamExt::flat_map) method."] FlatMap < St , U , F > (flatten :: Flatten < Map < St , F >, U >) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | flatten :: Flatten :: new (Map :: new (x , f))]) ;
    };
}

macro_388!();