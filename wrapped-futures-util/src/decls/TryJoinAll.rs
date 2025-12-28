macro_rules! deps {
    () => {
        TryJoinAllKind!();
    };
}

macro_rules! TryJoinAll {
    () => {
        deps!();
        # [doc = " Future for the [`try_join_all`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryJoinAll < F > where F : TryFuture , { kind : TryJoinAllKind < F > , }
    };
}

TryJoinAll!();