// Generated macro for impl_146 (impl)
macro_rules! Depcrateimpl_146 {
() => {
// Module: crate
// Provides: {"impl_146"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl zeroize :: Zeroize for PrivateKeyDer < 'static > { fn zeroize (& mut self) { match self { Self :: Pkcs1 (key) => key . zeroize () , Self :: Sec1 (key) => key . zeroize () , Self :: Pkcs8 (key) => key . zeroize () , } } }
};
}
