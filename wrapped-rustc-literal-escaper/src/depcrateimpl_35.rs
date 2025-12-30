// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl TryFrom < u8 > for MixedUnit { type Error = EscapeError ; # [inline] fn try_from (byte : u8) -> Result < Self , EscapeError > { NonZero :: new (byte) . map (From :: from) . ok_or (EscapeError :: NulInCStr) } }
};
}
