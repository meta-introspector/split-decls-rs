macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! HashSetExt {
    () => {
        deps!();
        # [doc = " A convenience extension trait to enable [`HashSet::new`] for hash sets that use `foldhash`."] pub trait HashSetExt { # [doc = " Creates an empty `HashSet`."] fn new () -> Self ; # [doc = " Creates an empty `HashSet` with at least the specified capacity."] fn with_capacity (capacity : usize) -> Self ; }
    };
}

HashSetExt!()