// Generated macro for left_scalar_mul_impl (macro)
macro_rules! Depcrate_geometry_quaternion_opsleft_scalar_mul_impl {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"left_scalar_mul_impl"}
// Dependencies: {}
macro_rules ! left_scalar_mul_impl (($ ($ T : ty) ,* $ (,) *) => { $ (impl Mul < Quaternion <$ T >> for $ T { type Output = Quaternion <$ T >; # [inline] fn mul (self , right : Quaternion <$ T >) -> Self :: Output { Quaternion :: from (self * right . coords) } } impl <'b > Mul <&'b Quaternion <$ T >> for $ T { type Output = Quaternion <$ T >; # [inline] fn mul (self , right : &'b Quaternion <$ T >) -> Self :: Output { Quaternion :: from (self * & right . coords) } }) * }) ;
};
}
