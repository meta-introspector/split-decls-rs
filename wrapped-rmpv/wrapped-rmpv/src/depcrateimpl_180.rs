// Generated macro for impl_180 (impl)
macro_rules! Depcrateimpl_180 {
() => {
// Module: crate
// Provides: {"impl_180"}
// Dependencies: {}
impl From < i16 > for Integer { # [inline] fn from (n : i16) -> Self { if n < 0 { Self { n : IntPriv :: NegInt (i64 :: from (n)) } } else { Self { n : IntPriv :: PosInt (n as u64) } } } }
};
}
