macro_rules! macro_176 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::for_each()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ForEachFuture < S , F > { # [pin] stream : S , f : F , } }
    };
}

macro_176!()