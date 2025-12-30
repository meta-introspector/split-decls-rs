// Generated macro for impl_246 (impl)
macro_rules! Depcrate_base_opsimpl_246 {
() => {
// Module: crate::base::ops
// Provides: {"impl_246"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorage < T , R , C > > Index < (usize , usize) > for Matrix < T , R , C , S > { type Output = T ; # [inline] fn index (& self , ij : (usize , usize)) -> & Self :: Output { let shape = self . shape () ; assert ! (ij . 0 < shape . 0 && ij . 1 < shape . 1 , "Matrix index out of bounds.") ; unsafe { self . get_unchecked ((ij . 0 , ij . 1)) } } }
};
}
