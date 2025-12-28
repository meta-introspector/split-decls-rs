macro_rules! deps {
    () => {
        Waker!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        # [doc = " Inner representation of a zero-capacity channel."] struct Inner { # [doc = " Senders waiting to pair up with a receive operation."] senders : Waker , # [doc = " Receivers waiting to pair up with a send operation."] receivers : Waker , # [doc = " Equals `true` when the channel is disconnected."] is_disconnected : bool , }
    };
}

Inner!()