macro_rules! macro_785 {
    () => {
        pin_project ! { # [doc = " Stream for the [poll_immediate](poll_immediate()) function."] # [doc = ""] # [doc = " It will never return [Poll::Pending](core::task::Poll::Pending)"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollImmediate < S > { # [pin] stream : Option < S > } }
    };
}

macro_785!();