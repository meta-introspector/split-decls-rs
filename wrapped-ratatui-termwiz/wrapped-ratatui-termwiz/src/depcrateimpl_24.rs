// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl FromTermwiz < Blink > for Modifier { fn from_termwiz (value : Blink) -> Self { match value { Blink :: None => Self :: empty () , Blink :: Slow => Self :: SLOW_BLINK , Blink :: Rapid => Self :: RAPID_BLINK , } } }
};
}
