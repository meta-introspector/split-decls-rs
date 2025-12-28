macro_rules! macro_4 {
    () => {
        pin_project ! { # [doc = " Future for the [`poll_once()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollOnce < F > { # [pin] f : F , } }
    };
}

macro_4!();