// Generated macro for impl_186 (impl)
macro_rules! Depcrate_bnimpl_186 {
() => {
// Module: crate::bn
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'b > Div < & 'b BigNumRef > for & BigNumRef { type Output = BigNum ; fn div (self , oth : & 'b BigNumRef) -> BigNum { let mut ctx = BigNumContext :: new () . unwrap () ; let mut r = BigNum :: new () . unwrap () ; r . checked_div (self , oth , & mut ctx) . unwrap () ; r } }
};
}
