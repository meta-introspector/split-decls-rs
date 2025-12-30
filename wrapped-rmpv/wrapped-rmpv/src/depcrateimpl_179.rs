// Generated macro for impl_179 (impl)
macro_rules! Depcrateimpl_179 {
() => {
// Module: crate
// Provides: {"impl_179"}
// Dependencies: {}
impl From < i8 > for Integer { # [inline] fn from (n : i8) -> Self { if n < 0 { Self { n : IntPriv :: NegInt (i64 :: from (n)) } } else { Self { n : IntPriv :: PosInt (n as u64) } } } }
};
}
