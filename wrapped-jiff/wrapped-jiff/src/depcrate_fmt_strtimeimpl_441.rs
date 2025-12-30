// Generated macro for impl_441 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_441 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_441"}
// Dependencies: {}
impl From < Timestamp > for BrokenDownTime { fn from (ts : Timestamp) -> BrokenDownTime { let dt = Offset :: UTC . to_datetime (ts) ; BrokenDownTime { offset : Some (Offset :: UTC) , timestamp : Some (ts) , .. BrokenDownTime :: from (dt) } } }
};
}
