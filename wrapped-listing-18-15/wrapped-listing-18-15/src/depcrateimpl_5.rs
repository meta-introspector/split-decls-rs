// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl State for Draft { fn request_review (self : Box < Self >) -> Box < dyn State > { Box :: new (PendingReview { }) } }
};
}
