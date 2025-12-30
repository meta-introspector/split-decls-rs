// Generated macro for impl_182 (impl)
macro_rules! Depcrateimpl_182 {
() => {
// Module: crate
// Provides: {"impl_182"}
// Dependencies: {}
impl From < i64 > for Integer { # [inline] fn from (n : i64) -> Self { if n < 0 { Self { n : IntPriv :: NegInt (n) } } else { Self { n : IntPriv :: PosInt (n as u64) } } } }
};
}
