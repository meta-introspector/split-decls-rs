macro_rules! deps {
    () => {
        TryReserveError!();
        AllocError!();
    };
}

macro_rules! TryReserveErrorKind {
    () => {
        deps!();
        # [doc = " Details of the allocation that caused a `TryReserveError`"] # [derive (Clone , PartialEq , Eq , Debug)] pub enum TryReserveErrorKind { # [doc = " Error due to the computed capacity exceeding the collection's maximum"] # [doc = " (usually `isize::MAX` bytes)."] CapacityOverflow , # [doc = " The memory allocator returned an error"] AllocError { # [doc = " The layout of allocation request that failed"] layout : Layout , # [doc (hidden)] non_exhaustive : () , } , }
    };
}

TryReserveErrorKind!()