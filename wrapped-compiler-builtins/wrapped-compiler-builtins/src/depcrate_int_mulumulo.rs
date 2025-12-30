// Generated macro for UMulo (trait)
macro_rules! Depcrate_int_mulUMulo {
() => {
// Module: crate::int::mul
// Provides: {"UMulo"}
// Dependencies: {}
pub (crate) trait UMulo : DInt + Int { fn mulo (self , rhs : Self) -> (Self , bool) { match (self . hi () . is_zero () , rhs . hi () . is_zero ()) { (false , false) => (self . wrapping_mul (rhs) , true) , (true , false) => { let mul_lo = self . lo () . widen_mul (rhs . lo ()) ; let mul_hi = self . lo () . widen_mul (rhs . hi ()) ; let (mul , o) = mul_lo . overflowing_add (mul_hi . lo () . widen_hi ()) ; (mul , o || ! mul_hi . hi () . is_zero ()) } (false , true) => { let mul_lo = rhs . lo () . widen_mul (self . lo ()) ; let mul_hi = rhs . lo () . widen_mul (self . hi ()) ; let (mul , o) = mul_lo . overflowing_add (mul_hi . lo () . widen_hi ()) ; (mul , o || ! mul_hi . hi () . is_zero ()) } (true , true) => (self . lo () . widen_mul (rhs . lo ()) , false) , } } }
};
}
