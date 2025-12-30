// Generated macro for impl_914 (impl)
macro_rules! Depcrate_base_matriximpl_914 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_914"}
// Dependencies: {}
impl < T , D : Dim , S : RawStorage < T , D > > Vector < T , D , S > { # [doc = " Gets a reference to the i-th element of this column vector without bound checking."] # [doc = " # Safety"] # [doc = " `i` must be less than `D`."] # [inline] # [must_use] pub unsafe fn vget_unchecked (& self , i : usize) -> & T { debug_assert ! (i < self . nrows () , "Vector index out of bounds.") ; let i = i * self . strides () . 0 ; self . data . get_unchecked_linear (i) } }
};
}
