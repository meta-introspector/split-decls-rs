macro_rules! Key {
    () => {
        pub struct Key < K , V , P = (K , V) > { _phantom : PhantomData < (K , V , P) > , }
    };
}

Key!();