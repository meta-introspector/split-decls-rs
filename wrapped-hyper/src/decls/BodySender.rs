macro_rules! deps {
    () => {
        Result!();
        Error!();
        Sender!();
    };
}

macro_rules! BodySender {
    () => {
        deps!();
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] type BodySender = mpsc :: Sender < Result < Bytes , crate :: Error > > ;
    };
}

BodySender!();