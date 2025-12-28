macro_rules! macro_98 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::collect()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CollectFuture < S , C > { # [pin] stream : S , collection : C , } }
    };
}

macro_98!()