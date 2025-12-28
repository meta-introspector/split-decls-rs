macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Id { pub (crate) fn new () -> Id { use std :: sync :: atomic :: AtomicUsize ; use std :: sync :: atomic :: Ordering :: Relaxed ; static NEXT_ID : AtomicUsize = AtomicUsize :: new (46_413_762) ; let next = NEXT_ID . fetch_add (1 , Relaxed) ; Id (next) } }
    };
}

impl_66!();