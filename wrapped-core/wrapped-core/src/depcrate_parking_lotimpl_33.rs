// Generated macro for impl_33 (impl)
macro_rules! Depcrate_parking_lotimpl_33 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_33"}
// Dependencies: {}
impl ParkResult { # [doc = " Returns true if we were unparked by another thread."] # [inline] pub fn is_unparked (self) -> bool { if let ParkResult :: Unparked (_) = self { true } else { false } } }
};
}
