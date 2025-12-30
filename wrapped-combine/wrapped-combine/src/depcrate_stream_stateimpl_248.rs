// Generated macro for impl_248 (impl)
macro_rules! Depcrate_stream_stateimpl_248 {
() => {
// Module: crate::stream::state
// Provides: {"impl_248"}
// Dependencies: {}
impl < S , U > StreamOnce for Stream < S , U > where S : StreamOnce , { type Token = S :: Token ; type Range = S :: Range ; type Position = S :: Position ; type Error = S :: Error ; # [inline] fn uncons (& mut self) -> Result < S :: Token , StreamErrorFor < Self > > { self . stream . uncons () } fn is_partial (& self) -> bool { self . stream . is_partial () } }
};
}
