macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < I , EFN , E > Iter < I , EFN > where I : Iterator , EFN : FnOnce () -> E , { # [doc = " Create a new iterator over `inner` which checks for interruptions on each iteration and calls `make_err()` to"] # [doc = " signal an interruption happened, causing no further items to be iterated from that point on."] pub fn new (inner : I , make_err : EFN) -> Self { Iter { inner : gix_features :: interrupt :: IterWithErr :: new (inner , make_err , & IS_INTERRUPTED) , } } # [doc = " Return the inner iterator"] pub fn into_inner (self) -> I { self . inner . inner } # [doc = " Return the inner iterator as reference"] pub fn inner (& self) -> & I { & self . inner . inner } }
    };
}

impl_2!()