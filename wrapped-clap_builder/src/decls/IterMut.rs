macro_rules! IterMut {
    () => {
        pub (crate) struct IterMut < 'a , K , V > { keys : std :: slice :: IterMut < 'a , K > , values : std :: slice :: IterMut < 'a , V > , }
    };
}

IterMut!()