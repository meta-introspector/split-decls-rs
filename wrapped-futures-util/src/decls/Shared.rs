macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Shared {
    () => {
        deps!();
        # [doc = " Future for the [`shared`](super::FutureExt::shared) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Shared < Fut : Future > { inner : Option < Arc < Inner < Fut > > > , waker_key : usize , }
    };
}

Shared!()