// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl From < NonZero < u8 > > for MixedUnit { # [inline] fn from (byte : NonZero < u8 >) -> Self { if byte . get () . is_ascii () { MixedUnit :: Char (NonZero :: new (byte . get () as char) . unwrap ()) } else { MixedUnit :: HighByte (byte) } } }
};
}
