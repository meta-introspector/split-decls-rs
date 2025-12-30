// Generated macro for impl_188 (impl)
macro_rules! Depcrate_bnimpl_188 {
() => {
// Module: crate::bn
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'b > Rem < & 'b BigNumRef > for & BigNumRef { type Output = BigNum ; fn rem (self , oth : & 'b BigNumRef) -> BigNum { let mut ctx = BigNumContext :: new () . unwrap () ; let mut r = BigNum :: new () . unwrap () ; r . checked_rem (self , oth , & mut ctx) . unwrap () ; r } }
};
}
