macro_rules! macro_100 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::try_collect()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryCollectFuture < S , C > { # [pin] stream : S , items : C , } }
    };
}

macro_100!();