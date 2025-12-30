// Generated macro for impl_114 (impl)
macro_rules! Depcrate_errimpl_114 {
() => {
// Module: crate::err
// Provides: {"impl_114"}
// Dependencies: {}
impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Empty => "receiving on an empty channel" . fmt (f) , Self :: Disconnected => "receiving on an empty and disconnected channel" . fmt (f) , } } }
};
}
