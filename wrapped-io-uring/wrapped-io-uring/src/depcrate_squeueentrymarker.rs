// Generated macro for EntryMarker (trait)
macro_rules! Depcrate_squeueEntryMarker {
() => {
// Module: crate::squeue
// Provides: {"EntryMarker"}
// Dependencies: {}
# [doc = " A submission queue entry (SQE), representing a request for an I/O operation."] # [doc = ""] # [doc = " This is implemented for [`Entry`] and [`Entry128`]."] pub trait EntryMarker : Clone + Debug + From < Entry > + private :: Sealed { const BUILD_FLAGS : u32 ; }
};
}
