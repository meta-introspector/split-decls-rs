macro_rules! macro_659 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_collect`](super::TryStreamExt::try_collect) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryCollect < St , C > { # [pin] stream : St , items : C , } }
    };
}

macro_659!();