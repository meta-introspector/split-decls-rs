// Generated macro for impl_182 (impl)
macro_rules! Depcrate_bnimpl_182 {
() => {
// Module: crate::bn
// Provides: {"impl_182"}
// Dependencies: {}
impl Sub < & BigNumRef > for & BigNumRef { type Output = BigNum ; fn sub (self , oth : & BigNumRef) -> BigNum { let mut r = BigNum :: new () . unwrap () ; r . checked_sub (self , oth) . unwrap () ; r } }
};
}
