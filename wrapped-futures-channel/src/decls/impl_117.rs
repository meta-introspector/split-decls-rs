macro_rules! deps {
    () => {
        Receiver!();
        Canceled!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T > Receiver < T > { # [doc = " Gracefully close this receiver, preventing any subsequent attempts to"] # [doc = " send to it."] # [doc = ""] # [doc = " Any `send` operation which happens after this method returns is"] # [doc = " guaranteed to fail. After calling this method, you can use"] # [doc = " [`Receiver::poll`](core::future::Future::poll) to determine whether a"] # [doc = " message had previously been sent."] pub fn close (& mut self) { self . inner . close_rx () } # [doc = " Attempts to receive a message outside of the context of a task."] # [doc = ""] # [doc = " Does not schedule a task wakeup or have any other side effects."] # [doc = ""] # [doc = " A return value of `None` must be considered immediately stale (out of"] # [doc = " date) unless [`close`](Receiver::close) has been called first."] # [doc = ""] # [doc = " Returns an error if the sender was dropped."] pub fn try_recv (& mut self) -> Result < Option < T > , Canceled > { self . inner . try_recv () } }
    };
}

impl_117!();