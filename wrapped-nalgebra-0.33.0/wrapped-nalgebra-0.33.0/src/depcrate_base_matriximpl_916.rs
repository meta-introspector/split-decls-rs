// Generated macro for impl_916 (impl)
macro_rules! Depcrate_base_matriximpl_916 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_916"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorage < T , R , C > + IsContiguous > Matrix < T , R , C , S > { # [doc = " Extracts a slice containing the entire matrix entries ordered column-by-columns."] # [inline] # [must_use] pub fn as_slice (& self) -> & [T] { unsafe { self . data . as_slice_unchecked () } } }
};
}
