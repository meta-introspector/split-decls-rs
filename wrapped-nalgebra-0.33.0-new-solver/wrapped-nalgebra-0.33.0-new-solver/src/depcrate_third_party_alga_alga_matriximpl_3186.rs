// Generated macro for impl_3186 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3186 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3186"}
// Dependencies: {}
impl < T : ComplexField + simba :: scalar :: ComplexField < RealField = < T as ComplexField > :: RealField > , R : DimName , C : DimName , > NormedSpace for OMatrix < T , R , C > where < T as ComplexField > :: RealField : simba :: scalar :: RealField , DefaultAllocator : Allocator < R , C > , { type RealField = < T as ComplexField > :: RealField ; type ComplexField = T ; # [inline] fn norm_squared (& self) -> < T as ComplexField > :: RealField { self . norm_squared () } # [inline] fn norm (& self) -> < T as ComplexField > :: RealField { self . norm () } # [inline] # [must_use = "Did you mean to use normalize_mut()?"] fn normalize (& self) -> Self { self . normalize () } # [inline] fn normalize_mut (& mut self) -> < T as ComplexField > :: RealField { self . normalize_mut () } # [inline] # [must_use = "Did you mean to use try_normalize_mut()?"] fn try_normalize (& self , min_norm : < T as ComplexField > :: RealField) -> Option < Self > { self . try_normalize (min_norm) } # [inline] fn try_normalize_mut (& mut self , min_norm : < T as ComplexField > :: RealField ,) -> Option < < T as ComplexField > :: RealField > { self . try_normalize_mut (min_norm) } }
};
}
