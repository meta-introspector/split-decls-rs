macro_rules! deps {
    () => {
        StoreSlice!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < K , V > StoreSlice < K , V > for & [(K , V)] { type Slice = [(K , V)] ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > { self . get (range) } }
    };
}

impl_70!()