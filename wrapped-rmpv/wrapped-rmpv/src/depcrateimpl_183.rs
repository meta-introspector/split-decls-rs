// Generated macro for impl_183 (impl)
macro_rules! Depcrateimpl_183 {
() => {
// Module: crate
// Provides: {"impl_183"}
// Dependencies: {}
impl From < isize > for Integer { # [inline] fn from (n : isize) -> Self { if n < 0 { Self { n : IntPriv :: NegInt (n as i64) } } else { Self { n : IntPriv :: PosInt (n as u64) } } } }
};
}
