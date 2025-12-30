// Generated macro for impl_192 (impl)
macro_rules! Depcrate_bnimpl_192 {
() => {
// Module: crate::bn
// Provides: {"impl_192"}
// Dependencies: {}
impl Shr < i32 > for & BigNumRef { type Output = BigNum ; fn shr (self , n : i32) -> BigNum { let mut r = BigNum :: new () . unwrap () ; r . rshift (self , n) . unwrap () ; r } }
};
}
