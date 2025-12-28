macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [doc = " Zero-capacity channel."] pub (crate) struct Channel < T > { # [doc = " Inner representation of the channel."] inner : Mutex < Inner > , # [doc = " Indicates that dropping a `Channel<T>` may drop values of type `T`."] _marker : PhantomData < T > , }
    };
}

Channel!();