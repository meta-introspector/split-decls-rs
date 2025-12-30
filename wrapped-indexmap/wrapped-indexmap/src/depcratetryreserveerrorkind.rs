// Generated macro for TryReserveErrorKind (enum)
macro_rules! DepcrateTryReserveErrorKind {
() => {
// Module: crate
// Provides: {"TryReserveErrorKind"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Debug)] enum TryReserveErrorKind { Std (alloc :: collections :: TryReserveError) , CapacityOverflow , AllocError { layout : alloc :: alloc :: Layout } , }
};
}
