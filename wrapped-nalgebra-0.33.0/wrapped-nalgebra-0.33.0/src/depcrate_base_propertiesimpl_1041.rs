// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_base_propertiesimpl_1041 {
() => {
// Module: crate::base::properties
// Provides: {"impl_1041"}
// Dependencies: {}
impl < T : ComplexField , R : Dim , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > { # [doc = " Checks that `Mᵀ × M = Id`."] # [doc = ""] # [doc = " In this definition `Id` is approximately equal to the identity matrix with a relative error"] # [doc = " equal to `eps`."] # [inline] # [must_use] pub fn is_orthogonal (& self , eps : T :: Epsilon) -> bool where T : Zero + One + ClosedAddAssign + ClosedMulAssign + RelativeEq , S : Storage < T , R , C > , T :: Epsilon : Clone , DefaultAllocator : Allocator < R , C > + Allocator < C , C > , { (self . ad_mul (self)) . is_identity (eps) } }
};
}
