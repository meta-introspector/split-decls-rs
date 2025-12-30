// Generated macro for impl_904 (impl)
macro_rules! Depcrate_base_matriximpl_904 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_904"}
// Dependencies: {}
impl < T , R , C , S > Matrix < T , R , C , S > { # [doc = " Creates a new matrix with the given data without statically checking that the matrix"] # [doc = " dimension matches the storage dimension."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The storage dimension must match the given dimensions."] # [inline (always)] pub const unsafe fn from_data_statically_unchecked (data : S) -> Matrix < T , R , C , S > { Matrix { data , _phantoms : PhantomData , } } }
};
}
