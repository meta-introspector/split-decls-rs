macro_rules! deps {
    () => {
        Sender!();
        Upgraded!();
        Result!();
    };
}

macro_rules! Pending {
    () => {
        deps!();
        # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] pub (super) struct Pending { tx : oneshot :: Sender < crate :: Result < Upgraded > > , }
    };
}

Pending!()