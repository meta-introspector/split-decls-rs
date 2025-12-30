// Generated macro for impl_181 (impl)
macro_rules! Depcrateimpl_181 {
() => {
// Module: crate
// Provides: {"impl_181"}
// Dependencies: {}
impl From < i32 > for Integer { # [inline] fn from (n : i32) -> Self { if n < 0 { Self { n : IntPriv :: NegInt (i64 :: from (n)) } } else { Self { n : IntPriv :: PosInt (n as u64) } } } }
};
}
