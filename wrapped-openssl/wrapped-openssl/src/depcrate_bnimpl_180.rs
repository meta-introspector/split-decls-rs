// Generated macro for impl_180 (impl)
macro_rules! Depcrate_bnimpl_180 {
() => {
// Module: crate::bn
// Provides: {"impl_180"}
// Dependencies: {}
impl Add < & BigNumRef > for & BigNumRef { type Output = BigNum ; fn add (self , oth : & BigNumRef) -> BigNum { let mut r = BigNum :: new () . unwrap () ; r . checked_add (self , oth) . unwrap () ; r } }
};
}
