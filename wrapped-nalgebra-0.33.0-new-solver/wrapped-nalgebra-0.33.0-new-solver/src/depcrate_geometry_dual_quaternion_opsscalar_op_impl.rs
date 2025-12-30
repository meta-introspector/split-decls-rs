// Generated macro for scalar_op_impl (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsscalar_op_impl {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"scalar_op_impl"}
// Dependencies: {}
macro_rules ! scalar_op_impl (($ ($ Op : ident , $ op : ident , $ OpAssign : ident , $ op_assign : ident) ;* $ (;) *) => { $ (impl < T : SimdRealField > $ Op < T > for DualQuaternion < T > where T :: Element : SimdRealField { type Output = DualQuaternion < T >; # [inline] fn $ op (self , n : T) -> Self :: Output { DualQuaternion :: from_real_and_dual (self . real . clone () .$ op (n . clone ()) , self . dual . clone () .$ op (n)) } } impl <'a , T : SimdRealField > $ Op < T > for &'a DualQuaternion < T > where T :: Element : SimdRealField { type Output = DualQuaternion < T >; # [inline] fn $ op (self , n : T) -> Self :: Output { DualQuaternion :: from_real_and_dual (self . real . clone () .$ op (n . clone ()) , self . dual . clone () .$ op (n)) } } impl < T : SimdRealField > $ OpAssign < T > for DualQuaternion < T > where T :: Element : SimdRealField { # [inline] fn $ op_assign (& mut self , n : T) { self . real .$ op_assign (n . clone ()) ; self . dual .$ op_assign (n) ; } }) * }) ;
};
}
