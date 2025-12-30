// Generated macro for impl_185 (impl)
macro_rules! Depcrate_stream_easyimpl_185 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_185"}
// Dependencies: {}
impl < S > StreamOnce for Stream < S > where S : StreamOnce + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , { type Token = S :: Token ; type Range = S :: Range ; type Position = S :: Position ; type Error = ParseError < S > ; # [inline] fn uncons (& mut self) -> Result < Self :: Token , StreamErrorFor < Self > > { self . 0 . uncons () . map_err (StreamError :: into_other) } fn is_partial (& self) -> bool { self . 0 . is_partial () } }
};
}
