// Generated macro for impl_195 (impl)
macro_rules! Depcrate_nonblockimpl_195 {
() => {
// Module: crate::nonblock
// Provides: {"impl_195"}
// Dependencies: {}
impl < T : 'static > MethodReply < T > { # [doc = " Convenience combinator in case you want to post-process the result after reading it"] pub fn and_then < T2 > (self , f : impl FnOnce (T) -> Result < T2 , Error > + Send + Sync + 'static) -> MethodReply < T2 > { MethodReply (Box :: pin (async move { let x = self . 0 . await ? ; f (x) })) } }
};
}
