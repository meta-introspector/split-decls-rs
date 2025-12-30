// Generated macro for mul_add_impl (macro)
macro_rules! Depcrate_ops_mul_addmul_add_impl {
() => {
// Module: crate::ops::mul_add
// Provides: {"mul_add_impl"}
// Dependencies: {}
macro_rules ! mul_add_impl { ($ trait_name : ident for $ ($ t : ty) *) => { $ (impl $ trait_name for $ t { type Output = Self ; # [inline] fn mul_add (self , a : Self , b : Self) -> Self :: Output { (self * a) + b } }) * } }
};
}
