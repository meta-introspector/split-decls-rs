// Generated macro for impl_248 (impl)
macro_rules! Depcrate_base_opsimpl_248 {
() => {
// Module: crate::base::ops
// Provides: {"impl_248"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > IndexMut < (usize , usize) > for Matrix < T , R , C , S > { # [inline] fn index_mut (& mut self , ij : (usize , usize)) -> & mut T { let shape = self . shape () ; assert ! (ij . 0 < shape . 0 && ij . 1 < shape . 1 , "Matrix index out of bounds.") ; unsafe { self . get_unchecked_mut ((ij . 0 , ij . 1)) } } }
};
}
