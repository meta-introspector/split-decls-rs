macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < T , A > Default for HashTable < T , A > where A : Allocator + Default , { fn default () -> Self { Self { raw : Default :: default () , } } }
    };
}

impl_474!()