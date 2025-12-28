macro_rules! macro_66 {
    () => {
        pin_project ! { # [doc = " Future for the [`catch_unwind`](super::FutureExt::catch_unwind) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CatchUnwind < Fut > { # [pin] future : Fut , } }
    };
}

macro_66!()