// Generated macro for impl_820 (impl)
macro_rules! Depcrate_base_editionimpl_820 {
() => {
// Module: crate::base::edition
// Provides: {"impl_820"}
// Dependencies: {}
# [doc = " # In-place swapping"] impl < T : Scalar , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > Matrix < T , R , C , S > { # [doc = " Swaps two rows in-place."] # [inline] pub fn swap_rows (& mut self , irow1 : usize , irow2 : usize) { assert ! (irow1 < self . nrows () && irow2 < self . nrows ()) ; if irow1 != irow2 { for i in 0 .. self . ncols () { unsafe { self . swap_unchecked ((irow1 , i) , (irow2 , i)) } } } } # [doc = " Swaps two columns in-place."] # [inline] pub fn swap_columns (& mut self , icol1 : usize , icol2 : usize) { assert ! (icol1 < self . ncols () && icol2 < self . ncols ()) ; if icol1 != icol2 { for i in 0 .. self . nrows () { unsafe { self . swap_unchecked ((i , icol1) , (i , icol2)) } } } } }
};
}
