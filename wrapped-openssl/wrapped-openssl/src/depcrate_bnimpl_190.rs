// Generated macro for impl_190 (impl)
macro_rules! Depcrate_bnimpl_190 {
() => {
// Module: crate::bn
// Provides: {"impl_190"}
// Dependencies: {}
impl Shl < i32 > for & BigNumRef { type Output = BigNum ; fn shl (self , n : i32) -> BigNum { let mut r = BigNum :: new () . unwrap () ; r . lshift (self , n) . unwrap () ; r } }
};
}
