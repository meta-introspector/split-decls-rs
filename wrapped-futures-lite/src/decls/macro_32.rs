macro_rules! macro_32 {
    () => {
        # [cfg (feature = "std")] pin_project ! { # [doc = " Future for the [`FutureExt::catch_unwind()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CatchUnwind < F > { # [pin] inner : F , } }
    };
}

macro_32!();