macro_rules! deps {
    () => {
        Vec!();
        BinaryHeap!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < T , K , const N : usize > BinaryHeap < T , K , N > { # [doc = " Returns the underlying `Vec<T,N>`. Order is arbitrary and time is *O*(1)."] pub fn into_vec (self) -> Vec < T , N , usize > { self . data } }
    };
}

impl_368!();