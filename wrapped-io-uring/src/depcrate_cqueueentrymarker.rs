// Generated macro for EntryMarker (trait)
macro_rules! Depcrate_cqueueEntryMarker {
() => {
// Module: crate::cqueue
// Provides: {"EntryMarker"}
// Dependencies: {}
# [doc = " A completion queue entry (CQE), representing a complete I/O operation."] # [doc = ""] # [doc = " This is implemented for [`Entry`] and [`Entry32`]."] pub trait EntryMarker : Clone + Debug + Into < Entry > + private :: Sealed { const BUILD_FLAGS : u32 ; }
};
}
