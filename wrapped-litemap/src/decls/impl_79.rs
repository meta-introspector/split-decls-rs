macro_rules! deps {
    () => {
        StoreSlice!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < K , V > StoreSlice < K , V > for Vec < (K , V) > { type Slice = [(K , V)] ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > { self . get (range) } }
    };
}

impl_79!()