macro_rules! macro_185 {
    () => {
        pin_project ! { # [doc = " Future for the [`poll_immediate`](poll_immediate()) function."] # [doc = ""] # [doc = " It will never return [Poll::Pending](core::task::Poll::Pending)"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollImmediate < T > { # [pin] future : Option < T > } }
    };
}

macro_185!();