macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! StoreSlice {
    () => {
        deps!();
        pub trait StoreSlice < K : ? Sized , V : ? Sized > : Store < K , V > { type Slice : ? Sized ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > ; }
    };
}

StoreSlice!();