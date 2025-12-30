// Generated macro for impl_915 (impl)
macro_rules! Depcrate_base_matriximpl_915 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_915"}
// Dependencies: {}
impl < T , D : Dim , S : RawStorageMut < T , D > > Vector < T , D , S > { # [doc = " Gets a mutable reference to the i-th element of this column vector without bound checking."] # [doc = " # Safety"] # [doc = " `i` must be less than `D`."] # [inline] # [must_use] pub unsafe fn vget_unchecked_mut (& mut self , i : usize) -> & mut T { debug_assert ! (i < self . nrows () , "Vector index out of bounds.") ; let i = i * self . strides () . 0 ; self . data . get_unchecked_linear_mut (i) } }
};
}
