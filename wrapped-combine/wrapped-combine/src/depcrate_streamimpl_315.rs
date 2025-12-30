// Generated macro for impl_315 (impl)
macro_rules! Depcrate_streamimpl_315 {
() => {
// Module: crate::stream
// Provides: {"impl_315"}
// Dependencies: {}
impl < S > StreamOnce for CompleteStream < S > where S : StreamOnce , { type Token = S :: Token ; type Range = S :: Range ; type Position = S :: Position ; type Error = S :: Error ; # [inline] fn uncons (& mut self) -> Result < S :: Token , StreamErrorFor < Self > > { self . 0 . uncons () } fn is_partial (& self) -> bool { false } }
};
}
