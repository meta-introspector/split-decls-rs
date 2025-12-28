macro_rules! NoCacheImpl {
    () => {
        struct NoCacheImpl < K , V > { _mark1 : PhantomData < K > , _mark2 : PhantomData < V > , }
    };
}

NoCacheImpl!();