macro_rules! deps {
    () => {
        Buffer!();
        Murmur3Hasher!();
        Index!();
        State!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Default for Murmur3Hasher { fn default () -> Self { Self { buf : Buffer { bytes : MaybeUninit :: uninit () , } , index : Index :: _0 , processed : 0 , state : State (0) , } } }
    };
}

impl_14!();