macro_rules! deps {
    () => {
        StreamExt!();
        State!();
    };
}

macro_rules! WaitUntil {
    () => {
        deps!();
        # [doc = " Delay execution of a stream once for the specified duration."] # [doc = ""] # [doc = " This `struct` is created by the [`wait_until`] method on [`StreamExt`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`wait_until`]: crate::stream::StreamExt::wait_until"] # [doc = " [`StreamExt`]: crate::stream::StreamExt"] # [derive (Debug)] # [must_use = "streams do nothing unless polled or .awaited"] # [pin_project] pub struct WaitUntil < S , D > { # [pin] stream : S , # [pin] deadline : D , state : State , }
    };
}

WaitUntil!();