// Generated macro for impl_184 (impl)
macro_rules! Depcrate_bnimpl_184 {
() => {
// Module: crate::bn
// Provides: {"impl_184"}
// Dependencies: {}
impl Mul < & BigNumRef > for & BigNumRef { type Output = BigNum ; fn mul (self , oth : & BigNumRef) -> BigNum { let mut ctx = BigNumContext :: new () . unwrap () ; let mut r = BigNum :: new () . unwrap () ; r . checked_mul (self , oth , & mut ctx) . unwrap () ; r } }
};
}
