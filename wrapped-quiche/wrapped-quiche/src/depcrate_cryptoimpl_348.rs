// Generated macro for impl_348 (impl)
macro_rules! Depcrate_cryptoimpl_348 {
() => {
// Module: crate::crypto
// Provides: {"impl_348"}
// Dependencies: {}
impl Level { pub fn from_epoch (e : packet :: Epoch) -> Level { match e { packet :: Epoch :: Initial => Level :: Initial , packet :: Epoch :: Handshake => Level :: Handshake , packet :: Epoch :: Application => Level :: OneRTT , } } }
};
}
