// Generated macro for impl_944 (impl)
macro_rules! Depcrate_base_matriximpl_944 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_944"}
// Dependencies: {}
impl < T : Scalar + Field , S : RawStorage < T , U3 > > Vector < T , U3 , S > { # [doc = " Computes the matrix `M` such that for all vector `v` we have `M * v == self.cross(&v)`."] # [inline] # [must_use] pub fn cross_matrix (& self) -> OMatrix < T , U3 , U3 > { OMatrix :: < T , U3 , U3 > :: new (T :: zero () , - self [2] . clone () , self [1] . clone () , self [2] . clone () , T :: zero () , - self [0] . clone () , - self [1] . clone () , self [0] . clone () , T :: zero () ,) } }
};
}
