macro_rules! macro_59 {
    () => {
        delegate_all ! (# [doc = " Future for the [`map`](super::FutureExt::map) method."] Map < Fut , F > (map :: Map < Fut , F >) : Debug + Future + FusedFuture + New [| x : Fut , f : F | map :: Map :: new (x , f)]) ;
    };
}

macro_59!()