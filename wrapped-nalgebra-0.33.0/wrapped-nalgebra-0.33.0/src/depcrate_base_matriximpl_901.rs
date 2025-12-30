// Generated macro for impl_901 (impl)
macro_rules! Depcrate_base_matriximpl_901 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_901"}
// Dependencies: {}
# [cfg (feature = "compare")] impl < T : Scalar , R : Dim , C : Dim , S : RawStorage < T , R , C > > matrixcompare_core :: DenseAccess < T > for Matrix < T , R , C , S > { fn fetch_single (& self , row : usize , col : usize) -> T { self . index ((row , col)) . clone () } }
};
}
