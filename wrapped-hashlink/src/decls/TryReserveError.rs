macro_rules! TryReserveError {
    () => {
        pub enum TryReserveError { CapacityOverflow , AllocError { layout : Layout } , }
    };
}

TryReserveError!()