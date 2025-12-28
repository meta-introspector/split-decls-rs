macro_rules! deps {
    () => {
        AllObjects!();
        Ordering!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [doc = " Builder"] impl AllObjects { # [doc = " Set the ordering of the objects returned, trading off memory and latency for object query performance."] pub fn with_ordering (mut self , order : Ordering) -> Self { self . order = order ; self } }
    };
}

impl_21!();