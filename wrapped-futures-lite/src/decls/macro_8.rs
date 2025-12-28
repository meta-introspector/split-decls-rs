macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`poll_fn()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollFn < F > { f : F , } }
    };
}

macro_8!()