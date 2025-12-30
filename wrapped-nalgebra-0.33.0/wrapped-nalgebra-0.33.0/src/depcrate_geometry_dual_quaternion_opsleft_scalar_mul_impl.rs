// Generated macro for left_scalar_mul_impl (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsleft_scalar_mul_impl {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"left_scalar_mul_impl"}
// Dependencies: {}
macro_rules ! left_scalar_mul_impl (($ ($ T : ty) ,* $ (,) *) => { $ (impl Mul < DualQuaternion <$ T >> for $ T { type Output = DualQuaternion <$ T >; # [inline] fn mul (self , right : DualQuaternion <$ T >) -> Self :: Output { DualQuaternion :: from_real_and_dual (self * right . real , self * right . dual) } } impl <'b > Mul <&'b DualQuaternion <$ T >> for $ T { type Output = DualQuaternion <$ T >; # [inline] fn mul (self , right : &'b DualQuaternion <$ T >) -> Self :: Output { DualQuaternion :: from_real_and_dual (self * & right . real , self * & right . dual) } }) * }) ;
};
}
