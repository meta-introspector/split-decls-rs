macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! macro_424 {
    () => {
        deps!();
        peeking_next_by_clone ! { [I : Clone + PeekingNext + DoubleEndedIterator] :: std :: iter :: Rev < I > }
    };
}

macro_424!()