macro_rules! Iter {
    () => {
        pub (crate) struct Iter < 'a , K , V > { keys : std :: slice :: Iter < 'a , K > , values : std :: slice :: Iter < 'a , V > , }
    };
}

Iter!();