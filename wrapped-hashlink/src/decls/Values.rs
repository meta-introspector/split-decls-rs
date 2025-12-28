macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        pub struct Values < 'a , K , V > { inner : Iter < 'a , K , V > , }
    };
}

Values!();