macro_rules! macro_46 {
    () => {
        pin_project ! { # [doc = " Future for the [`fuse`](super::FutureExt::fuse) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Fuse < Fut > { # [pin] inner : Option < Fut >, } }
    };
}

macro_46!()