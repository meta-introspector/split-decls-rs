macro_rules! StoreFromIterator {
    () => {
        # [doc = " A store that can be built from a tuple iterator."] pub trait StoreFromIterator < K , V > : FromIterator < (K , V) > { }
    };
}

StoreFromIterator!()