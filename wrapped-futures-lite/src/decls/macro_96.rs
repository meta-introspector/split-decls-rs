macro_rules! macro_96 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::count()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CountFuture < S : ? Sized > { count : usize , # [pin] stream : S , } }
    };
}

macro_96!();