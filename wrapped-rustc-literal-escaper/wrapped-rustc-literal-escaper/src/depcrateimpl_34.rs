// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl TryFrom < char > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (c : char) -> Result < Self , EscapeError > { NonZero :: new (c) . map (MixedUnit :: Char) . ok_or (EscapeError :: NulInCStr) } }
};
}
