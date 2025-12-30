// Generated macro for impl_320 (impl)
macro_rules! Depcrate_streamimpl_320 {
() => {
// Module: crate::stream
// Provides: {"impl_320"}
// Dependencies: {}
impl < S > StreamOnce for MaybePartialStream < S > where S : StreamOnce , { type Token = S :: Token ; type Range = S :: Range ; type Position = S :: Position ; type Error = S :: Error ; # [inline] fn uncons (& mut self) -> Result < S :: Token , StreamErrorFor < Self > > { self . 0 . uncons () } fn is_partial (& self) -> bool { self . 1 } }
};
}
