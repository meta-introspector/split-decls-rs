macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! Receiver {
    () => {
        deps!();
        # [doc = " Receiver handle to a channel."] pub (crate) struct Receiver < 'a , T > (& 'a Channel < T >) ;
    };
}

Receiver!()