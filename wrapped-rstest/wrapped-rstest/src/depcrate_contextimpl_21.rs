// Generated macro for impl_21 (impl)
macro_rules! Depcrate_contextimpl_21 {
() => {
// Module: crate::context
// Provides: {"impl_21"}
// Dependencies: {}
impl Context { # [doc = " Create a new test context. This function set also the start time to the current time."] pub fn new (module : & 'static str , name : & 'static str , description : Option < & 'static str > , case : Option < usize > ,) -> Self { Self { module , name , description , case , start : std :: time :: Instant :: now () , } } }
};
}
