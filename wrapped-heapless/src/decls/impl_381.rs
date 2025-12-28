macro_rules! deps {
    () => {
        Kind!();
        BinaryHeap!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl < T , K , const N : usize > Default for BinaryHeap < T , K , N > where T : Ord , K : Kind , { fn default () -> Self { Self :: new () } }
    };
}

impl_381!()