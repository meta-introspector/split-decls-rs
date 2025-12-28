macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Take!();
    };
}

macro_rules! impl_872 {
    () => {
        deps!();
        impl < I > Take < I > where I : IndexedParallelIterator , { # [doc = " Creates a new `Take` iterator."] pub (super) fn new (base : I , n : usize) -> Self { let n = Ord :: min (base . len () , n) ; Take { base , n } } }
    };
}

impl_872!()