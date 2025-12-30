// Generated macro for componentwise_scalarop_impl (macro)
macro_rules! Depcrate_base_opscomponentwise_scalarop_impl {
() => {
// Module: crate::base::ops
// Provides: {"componentwise_scalarop_impl"}
// Dependencies: {}
macro_rules ! componentwise_scalarop_impl (($ Trait : ident , $ method : ident , $ bound : ident ; $ TraitAssign : ident , $ method_assign : ident) => { impl < T , R : Dim , C : Dim , S > $ Trait < T > for Matrix < T , R , C , S > where T : Scalar + $ bound , S : Storage < T , R , C >, DefaultAllocator : Allocator < R , C > { type Output = OMatrix < T , R , C >; # [inline] fn $ method (self , rhs : T) -> Self :: Output { let mut res = self . into_owned () ; for left in res . as_mut_slice () . iter_mut () { * left = left . clone () .$ method (rhs . clone ()) } res } } impl <'a , T , R : Dim , C : Dim , S > $ Trait < T > for &'a Matrix < T , R , C , S > where T : Scalar + $ bound , S : Storage < T , R , C >, DefaultAllocator : Allocator < R , C > { type Output = OMatrix < T , R , C >; # [inline] fn $ method (self , rhs : T) -> Self :: Output { self . clone_owned () .$ method (rhs) } } impl < T , R : Dim , C : Dim , S > $ TraitAssign < T > for Matrix < T , R , C , S > where T : Scalar + $ bound , S : StorageMut < T , R , C > { # [inline] fn $ method_assign (& mut self , rhs : T) { for j in 0 .. self . ncols () { for i in 0 .. self . nrows () { unsafe { self . get_unchecked_mut ((i , j)) .$ method_assign (rhs . clone ()) } ; } } } } }) ;
};
}
