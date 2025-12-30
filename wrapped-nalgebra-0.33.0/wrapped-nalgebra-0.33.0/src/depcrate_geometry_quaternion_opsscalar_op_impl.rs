// Generated macro for scalar_op_impl (macro)
macro_rules! Depcrate_geometry_quaternion_opsscalar_op_impl {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"scalar_op_impl"}
// Dependencies: {}
macro_rules ! scalar_op_impl (($ ($ Op : ident , $ op : ident , $ OpAssign : ident , $ op_assign : ident) ;* $ (;) *) => { $ (impl < T : SimdRealField > $ Op < T > for Quaternion < T > where T :: Element : SimdRealField { type Output = Quaternion < T >; # [inline] fn $ op (self , n : T) -> Self :: Output { Quaternion :: from (self . coords .$ op (n)) } } impl <'a , T : SimdRealField > $ Op < T > for &'a Quaternion < T > where T :: Element : SimdRealField { type Output = Quaternion < T >; # [inline] fn $ op (self , n : T) -> Self :: Output { Quaternion :: from ((& self . coords) .$ op (n)) } } impl < T : SimdRealField > $ OpAssign < T > for Quaternion < T > where T :: Element : SimdRealField { # [inline] fn $ op_assign (& mut self , n : T) { self . coords .$ op_assign (n) } }) * }) ;
};
}
