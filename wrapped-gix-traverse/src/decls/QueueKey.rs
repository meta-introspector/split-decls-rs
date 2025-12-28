macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! QueueKey {
    () => {
        deps!();
        type QueueKey < T > = Either < T , Reverse < T > > ;
    };
}

QueueKey!()