macro_rules! PendingOnce {
    () => {
        # [doc = " Combinator that guarantees one [`Poll::Pending`] before polling its inner"] # [doc = " future."] # [doc = ""] # [doc = " This is created by the"] # [doc = " [`FutureTestExt::pending_once`](super::FutureTestExt::pending_once)"] # [doc = " method."] # [pin_project] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PendingOnce < Fut > { # [pin] future : Fut , polled_before : bool , }
    };
}

PendingOnce!();