macro_rules! macro_104 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::fold()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FoldFuture < S , F , T > { # [pin] stream : S , f : F , acc : Option < T >, } }
    };
}

macro_104!()