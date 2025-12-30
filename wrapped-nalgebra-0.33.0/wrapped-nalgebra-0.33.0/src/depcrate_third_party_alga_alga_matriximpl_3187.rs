// Generated macro for impl_3187 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3187 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3187"}
// Dependencies: {}
impl < T : ComplexField + simba :: scalar :: ComplexField < RealField = < T as ComplexField > :: RealField > , R : DimName , C : DimName , > InnerSpace for OMatrix < T , R , C > where < T as ComplexField > :: RealField : simba :: scalar :: RealField , DefaultAllocator : Allocator < R , C > , { # [inline] fn angle (& self , other : & Self) -> < T as ComplexField > :: RealField { self . angle (other) } # [inline] fn inner_product (& self , other : & Self) -> T { self . dotc (other) } }
};
}
