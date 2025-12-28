macro_rules! deps {
    () => {
        Cancellation!();
        Receiver!();
        Sender!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T > Sender < T > { # [doc = " Completes this oneshot with a successful result."] # [doc = ""] # [doc = " This function will consume `self` and indicate to the other end, the"] # [doc = " [`Receiver`], that the value provided is the result of the computation"] # [doc = " this represents."] # [doc = ""] # [doc = " If the value is successfully enqueued for the remote end to receive,"] # [doc = " then `Ok(())` is returned. If the receiving end was dropped before"] # [doc = " this function was called, however, then `Err(t)` is returned."] pub fn send (self , t : T) -> Result < () , T > { self . inner . send (t) } # [doc = " Polls this `Sender` half to detect whether its associated"] # [doc = " [`Receiver`] has been dropped."] # [doc = ""] # [doc = " # Return values"] # [doc = ""] # [doc = " If `Ready(())` is returned then the associated `Receiver` has been"] # [doc = " dropped, which means any work required for sending should be canceled."] # [doc = ""] # [doc = " If `Pending` is returned then the associated `Receiver` is still"] # [doc = " alive and may be able to receive a message if sent. The current task,"] # [doc = " however, is scheduled to receive a notification if the corresponding"] # [doc = " `Receiver` goes away."] pub fn poll_canceled (& mut self , cx : & mut Context < '_ >) -> Poll < () > { self . inner . poll_canceled (cx) } # [doc = " Creates a future that resolves when this `Sender`'s corresponding"] # [doc = " [`Receiver`] half has hung up."] # [doc = ""] # [doc = " This is a utility wrapping [`poll_canceled`](Sender::poll_canceled)"] # [doc = " to expose a [`Future`]."] pub fn cancellation (& mut self) -> Cancellation < '_ , T > { Cancellation { inner : self } } # [doc = " Tests to see whether this `Sender`'s corresponding `Receiver`"] # [doc = " has been dropped."] # [doc = ""] # [doc = " Unlike [`poll_canceled`](Sender::poll_canceled), this function does not"] # [doc = " enqueue a task for wakeup upon cancellation, but merely reports the"] # [doc = " current state, which may be subject to concurrent modification."] pub fn is_canceled (& self) -> bool { self . inner . is_canceled () } # [doc = " Tests to see whether this `Sender` is connected to the given `Receiver`. That is, whether"] # [doc = " they were created by the same call to `channel`."] pub fn is_connected_to (& self , receiver : & Receiver < T >) -> bool { Arc :: ptr_eq (& self . inner , & receiver . inner) } }
    };
}

impl_109!();