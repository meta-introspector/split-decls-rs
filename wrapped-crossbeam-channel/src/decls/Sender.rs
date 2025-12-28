macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! Sender {
    () => {
        deps!();
        # [doc = " Sender handle to a channel."] pub (crate) struct Sender < 'a , T > (& 'a Channel < T >) ;
    };
}

Sender!();