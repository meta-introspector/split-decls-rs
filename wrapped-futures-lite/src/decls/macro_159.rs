macro_rules! macro_159 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::last()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct LastFuture < S : Stream > { # [pin] stream : S , last : Option < S :: Item >, } }
    };
}

macro_159!()