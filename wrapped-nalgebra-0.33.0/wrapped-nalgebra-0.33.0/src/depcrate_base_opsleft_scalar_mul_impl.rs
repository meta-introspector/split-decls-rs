// Generated macro for left_scalar_mul_impl (macro)
macro_rules! Depcrate_base_opsleft_scalar_mul_impl {
() => {
// Module: crate::base::ops
// Provides: {"left_scalar_mul_impl"}
// Dependencies: {}
macro_rules ! left_scalar_mul_impl (($ ($ T : ty) ,* $ (,) *) => { $ (impl < R : Dim , C : Dim , S : Storage <$ T , R , C >> Mul < Matrix <$ T , R , C , S >> for $ T where DefaultAllocator : Allocator < R , C > { type Output = OMatrix <$ T , R , C >; # [inline] fn mul (self , rhs : Matrix <$ T , R , C , S >) -> Self :: Output { let mut res = rhs . into_owned () ; for rhs in res . as_mut_slice () . iter_mut () { * rhs *= self } res } } impl <'b , R : Dim , C : Dim , S : Storage <$ T , R , C >> Mul <&'b Matrix <$ T , R , C , S >> for $ T where DefaultAllocator : Allocator < R , C > { type Output = OMatrix <$ T , R , C >; # [inline] fn mul (self , rhs : &'b Matrix <$ T , R , C , S >) -> Self :: Output { self * rhs . clone_owned () } }) * }) ;
};
}
