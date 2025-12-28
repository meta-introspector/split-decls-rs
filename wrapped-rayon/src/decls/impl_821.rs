macro_rules! deps {
    () => {
        Skip!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_821 {
    () => {
        deps!();
        impl < I > Skip < I > where I : IndexedParallelIterator , { # [doc = " Creates a new `Skip` iterator."] pub (super) fn new (base : I , n : usize) -> Self { let n = Ord :: min (base . len () , n) ; Skip { base , n } } }
    };
}

impl_821!()