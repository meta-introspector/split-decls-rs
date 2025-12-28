macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! macro_547 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)"] # [doc = " method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ForEachConcurrent < St , Fut , F > { # [pin] stream : Option < St >, f : F , futures : FuturesUnordered < Fut >, limit : Option < NonZeroUsize >, } }
    };
}

macro_547!()