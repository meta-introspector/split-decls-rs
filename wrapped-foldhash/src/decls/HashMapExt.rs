macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! HashMapExt {
    () => {
        deps!();
        # [doc = " A convenience extension trait to enable [`HashMap::new`] for hash maps that use `foldhash`."] pub trait HashMapExt { # [doc = " Creates an empty `HashMap`."] fn new () -> Self ; # [doc = " Creates an empty `HashMap` with at least the specified capacity."] fn with_capacity (capacity : usize) -> Self ; }
    };
}

HashMapExt!();