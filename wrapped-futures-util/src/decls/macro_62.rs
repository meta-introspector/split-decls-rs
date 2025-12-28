macro_rules! macro_62 {
    () => {
        delegate_all ! (# [doc = " Future for the [`then`](FutureExt::then) method."] Then < Fut1 , Fut2 , F > (flatten :: Flatten < Map < Fut1 , F >, Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 , y : F | flatten :: Flatten :: new (Map :: new (x , y))]) ;
    };
}

macro_62!()