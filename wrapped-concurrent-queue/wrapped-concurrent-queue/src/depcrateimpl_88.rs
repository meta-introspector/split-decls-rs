// Generated macro for impl_88 (impl)
macro_rules! Depcrateimpl_88 {
() => {
// Module: crate
// Provides: {"impl_88"}
// Dependencies: {}
impl < T > PushError < T > { # [doc = " Unwraps the item that couldn't be pushed."] pub fn into_inner (self) -> T { match self { PushError :: Full (t) => t , PushError :: Closed (t) => t , } } # [doc = " Returns `true` if the queue is full but not closed."] pub fn is_full (& self) -> bool { match self { PushError :: Full (_) => true , PushError :: Closed (_) => false , } } # [doc = " Returns `true` if the queue is closed."] pub fn is_closed (& self) -> bool { match self { PushError :: Full (_) => false , PushError :: Closed (_) => true , } } }
};
}
