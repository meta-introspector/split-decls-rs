// Generated macro for impl_949 (impl)
macro_rules! Depcrate_base_matriximpl_949 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_949"}
// Dependencies: {}
impl < T , R , C , S > Hash for Matrix < T , R , C , S > where T : Scalar + Hash , R : Dim , C : Dim , S : RawStorage < T , R , C > , { fn hash < H : Hasher > (& self , state : & mut H) { let (nrows , ncols) = self . shape () ; (nrows , ncols) . hash (state) ; for j in 0 .. ncols { for i in 0 .. nrows { unsafe { self . get_unchecked ((i , j)) . hash (state) ; } } } } }
};
}
