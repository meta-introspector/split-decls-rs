// Generated macro for impl_10 (impl)
macro_rules! Depcrate_acquireimpl_10 {
() => {
// Module: crate::acquire
// Provides: {"impl_10"}
// Dependencies: {}
impl fmt :: Display for Fail { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Fail :: Immediately => f . write_str ("immediately") , Fail :: AfterDurationWithBackoff (duration) => { write ! (f , "after {:.02}s" , duration . as_secs_f32 ()) } } } }
};
}
