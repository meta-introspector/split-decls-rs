macro_rules! macro_111 {
    () => {
        pin_project ! { # [doc = " Future for the [`into_future`](super::TryFutureExt::into_future) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct IntoFuture < Fut > { # [pin] future : Fut , } }
    };
}

macro_111!()