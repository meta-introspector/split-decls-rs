macro_rules! macro_57 {
    () => {
        delegate_all ! (# [doc = " Future for the [`flatten`](super::FutureExt::flatten) method."] Flatten < F > (flatten :: Flatten < F , < F as Future >:: Output >) : Debug + Future + FusedFuture + New [| x : F | flatten :: Flatten :: new (x)] where F : Future) ;
    };
}

macro_57!()