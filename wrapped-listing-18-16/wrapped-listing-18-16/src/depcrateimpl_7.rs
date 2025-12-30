// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl State for PendingReview { fn request_review (self : Box < Self >) -> Box < dyn State > { self } fn approve (self : Box < Self >) -> Box < dyn State > { Box :: new (Published { }) } }
};
}
