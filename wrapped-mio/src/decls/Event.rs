macro_rules! deps {
    () => {
        Token!();
        Poll!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        # [doc = " A readiness event."] # [doc = ""] # [doc = " `Event` is a readiness state paired with a [`Token`]. It is returned by"] # [doc = " [`Poll::poll`]."] # [doc = ""] # [doc = " For more documentation on polling and events, see [`Poll`]."] # [doc = ""] # [doc = " [`Poll::poll`]: ../struct.Poll.html#method.poll"] # [doc = " [`Poll`]: ../struct.Poll.html"] # [doc = " [`Token`]: ../struct.Token.html"] # [derive (Clone)] # [repr (transparent)] pub struct Event { inner : sys :: Event , }
    };
}

Event!();