macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , I > Iter < 'a , I > where I : Iterator , { # [doc = " Create a new iterator over `inner` which checks for interruptions on each iteration on `should_interrupt`."] # [doc = ""] # [doc = " Note that this means the consumer of the iterator data should also be able to access `should_interrupt` and"] # [doc = " consider it when producing the final result to avoid claiming success even though the operation is only partially"] # [doc = " complete."] pub fn new (inner : I , should_interrupt : & 'a AtomicBool) -> Self { Iter { inner , should_interrupt , } } }
    };
}

impl_16!();