macro_rules! StoreConstEmpty {
    () => {
        # [doc = " Trait to enable const construction of empty store."] pub trait StoreConstEmpty < K : ? Sized , V : ? Sized > { # [doc = " An empty store"] const EMPTY : Self ; }
    };
}

StoreConstEmpty!()