macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl LruCache { # [doc = " Creates a new LRU Cache that holds at most `cap` items."] pub fn new (cap : usize) -> Self { Self { cap } } }
    };
}

impl_373!()