macro_rules! deps {
    () => {
        Kind!();
        BinaryHeap!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < T , K , const N : usize > Clone for BinaryHeap < T , K , N > where K : Kind , T : Ord + Clone , { fn clone (& self) -> Self { Self { _kind : self . _kind , data : self . data . clone () , } } }
    };
}

impl_382!();