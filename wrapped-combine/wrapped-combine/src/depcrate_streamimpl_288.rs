// Generated macro for impl_288 (impl)
macro_rules! Depcrate_streamimpl_288 {
() => {
// Module: crate::stream
// Provides: {"impl_288"}
// Dependencies: {}
impl < 'a , I > StreamOnce for & 'a mut I where I : StreamOnce + ? Sized , { type Token = I :: Token ; type Range = I :: Range ; type Position = I :: Position ; type Error = I :: Error ; fn uncons (& mut self) -> Result < Self :: Token , StreamErrorFor < Self > > { (* * self) . uncons () } fn is_partial (& self) -> bool { (* * self) . is_partial () } }
};
}
