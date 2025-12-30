// Generated macro for impl_308 (impl)
macro_rules! Depcrate_streamimpl_308 {
() => {
// Module: crate::stream
// Provides: {"impl_308"}
// Dependencies: {}
impl < S > StreamOnce for PartialStream < S > where S : StreamOnce , { type Token = S :: Token ; type Range = S :: Range ; type Position = S :: Position ; type Error = S :: Error ; # [inline] fn uncons (& mut self) -> Result < S :: Token , StreamErrorFor < Self > > { self . 0 . uncons () } fn is_partial (& self) -> bool { true } }
};
}
