// Generated macro for impl_536 (impl)
macro_rules! Depcrate_utilsimpl_536 {
() => {
// Module: crate::utils
// Provides: {"impl_536"}
// Dependencies: {}
impl < A : Actor > IntervalFunc < A > { # [doc = " Constructs an `IntervalFunc` using the given `interval` duration and `task` function."] pub fn new < F > (interval : Duration , task : F) -> IntervalFunc < A > where F : FnMut (& mut A , & mut A :: Context) + 'static , { Self { f : Box :: new (task) , interval , timer : sleep (interval) , } } # [doc = " Constructs an `IntervalFunc` using the given `start` time, `interval` duration, and `task`"] # [doc = " function."] pub fn new_at < F > (start : Instant , interval : Duration , task : F) -> IntervalFunc < A > where F : FnMut (& mut A , & mut A :: Context) + 'static , { Self { f : Box :: new (task) , interval , timer : sleep_until (start) , } } }
};
}
