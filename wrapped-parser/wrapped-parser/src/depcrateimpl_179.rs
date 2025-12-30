// Generated macro for impl_179 (impl)
macro_rules! Depcrateimpl_179 {
() => {
// Module: crate
// Provides: {"impl_179"}
// Dependencies: {}
impl ExactSizeIterator for ErrorPositions { fn len (& self) -> usize { match self . 0 { ErrorPositionsInner :: Two (_ , _) => 2 , ErrorPositionsInner :: One (_) => 1 , ErrorPositionsInner :: None => 0 , } } }
};
}
