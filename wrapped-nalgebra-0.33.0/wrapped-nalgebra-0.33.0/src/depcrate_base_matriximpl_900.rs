// Generated macro for impl_900 (impl)
macro_rules! Depcrate_base_matriximpl_900 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_900"}
// Dependencies: {}
# [cfg (feature = "compare")] impl < T : Scalar , R : Dim , C : Dim , S : RawStorage < T , R , C > > matrixcompare_core :: Matrix < T > for Matrix < T , R , C , S > { fn rows (& self) -> usize { self . nrows () } fn cols (& self) -> usize { self . ncols () } fn access (& self) -> matrixcompare_core :: Access < '_ , T > { matrixcompare_core :: Access :: Dense (self) } }
};
}
