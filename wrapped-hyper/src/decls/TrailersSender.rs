macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! TrailersSender {
    () => {
        deps!();
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] type TrailersSender = oneshot :: Sender < HeaderMap > ;
    };
}

TrailersSender!()