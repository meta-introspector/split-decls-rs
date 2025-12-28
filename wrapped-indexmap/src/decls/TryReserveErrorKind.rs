macro_rules! deps {
    () => {
        TryReserveError!();
    };
}

macro_rules! TryReserveErrorKind {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] enum TryReserveErrorKind { Std (alloc :: collections :: TryReserveError) , CapacityOverflow , AllocError { layout : alloc :: alloc :: Layout } , }
    };
}

TryReserveErrorKind!()