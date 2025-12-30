// Generated macro for impl_118 (impl)
macro_rules! Depcrate_errimpl_118 {
() => {
// Module: crate::err
// Provides: {"impl_118"}
// Dependencies: {}
impl fmt :: Display for RecvTimeoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Timeout => "timed out waiting on receive operation" . fmt (f) , Self :: Disconnected => "channel is empty and disconnected" . fmt (f) , } } }
};
}
