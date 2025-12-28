macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! macro_572 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the"] # [doc = " [`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)"] # [doc = " method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEachConcurrent < St , Fut , F > { # [pin] stream : Option < St >, f : F , futures : FuturesUnordered < Fut >, limit : Option < NonZeroUsize >, } }
    };
}

macro_572!();