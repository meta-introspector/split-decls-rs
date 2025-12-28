macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! Keys {
    () => {
        deps!();
        pub struct Keys < 'a , K , V > { inner : Iter < 'a , K , V > , }
    };
}

Keys!();