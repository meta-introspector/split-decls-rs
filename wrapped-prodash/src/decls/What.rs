macro_rules! deps {
    () => {
        Unit!();
    };
}

macro_rules! What {
    () => {
        deps!();
        pub (crate) enum What { ValuesAndUnit , Unit , Values , }
    };
}

What!()