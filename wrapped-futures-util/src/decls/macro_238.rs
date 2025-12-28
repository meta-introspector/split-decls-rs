macro_rules! deps {
    () => {
        TryMaybeDone!();
    };
}

macro_rules! macro_238 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`try_join`](super::TryFutureExt::try_join) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryJoin < Fut1 : TryFuture , Fut2 : TryFuture > { # [pin] fut1 : TryMaybeDone < Fut1 >, # [pin] fut2 : TryMaybeDone < Fut2 > } }
    };
}

macro_238!();