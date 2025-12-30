// Generated macro for impl_533 (impl)
macro_rules! Depcrate_utilsimpl_533 {
() => {
// Module: crate::utils
// Provides: {"impl_533"}
// Dependencies: {}
impl < A : Actor > TimerFunc < A > { # [doc = " Creates a new `TimerFunc` with the given duration."] pub fn new < F > (timeout : Duration , f : F) -> TimerFunc < A > where F : FnOnce (& mut A , & mut A :: Context) + 'static , { TimerFunc { f : Some (Box :: new (f)) , timeout : sleep (timeout) , } } }
};
}
