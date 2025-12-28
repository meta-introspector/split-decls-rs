macro_rules! macro_80 {
    () => {
        pin_project ! { # [doc = " Stream for the [`once_future()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct OnceFuture < F > { # [pin] future : Option < F >, } }
    };
}

macro_80!();