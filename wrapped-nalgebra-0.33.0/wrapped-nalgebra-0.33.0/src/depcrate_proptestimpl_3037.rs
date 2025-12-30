// Generated macro for impl_3037 (impl)
macro_rules! Depcrate_proptestimpl_3037 {
() => {
// Module: crate::proptest
// Provides: {"impl_3037"}
// Dependencies: {}
impl < T , R , C > Arbitrary for OMatrix < T , R , C > where T : Scalar + Arbitrary , < T as Arbitrary > :: Strategy : Clone , R : Dim , C : Dim , MatrixParameters < T :: Parameters , R , C > : Default , DefaultAllocator : Allocator < R , C > , { type Parameters = MatrixParameters < T :: Parameters , R , C > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { let value_strategy = T :: arbitrary_with (args . value_parameters) ; matrix (value_strategy , args . rows , args . cols) } type Strategy = MatrixStrategy < T :: Strategy , R , C > ; }
};
}
