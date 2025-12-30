// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl CongestionAlgorithm { pub fn build (self) -> Arc < dyn ControllerFactory + Send + Sync + 'static > { match self { CongestionAlgorithm :: Cubic => Arc :: new (congestion :: CubicConfig :: default ()) , CongestionAlgorithm :: Bbr => Arc :: new (congestion :: BbrConfig :: default ()) , CongestionAlgorithm :: NewReno => Arc :: new (congestion :: NewRenoConfig :: default ()) , } } }
};
}
