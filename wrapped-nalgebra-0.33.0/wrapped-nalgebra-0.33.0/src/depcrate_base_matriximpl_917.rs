// Generated macro for impl_917 (impl)
macro_rules! Depcrate_base_matriximpl_917 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_917"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > + IsContiguous > Matrix < T , R , C , S > { # [doc = " Extracts a mutable slice containing the entire matrix entries ordered column-by-columns."] # [inline] # [must_use] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { self . data . as_mut_slice_unchecked () } } }
};
}
