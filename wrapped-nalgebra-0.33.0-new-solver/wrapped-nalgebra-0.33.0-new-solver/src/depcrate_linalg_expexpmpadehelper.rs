// Generated macro for ExpmPadeHelper (struct)
macro_rules! Depcrate_linalg_expExpmPadeHelper {
() => {
// Module: crate::linalg::exp
// Provides: {"ExpmPadeHelper"}
// Dependencies: {}
struct ExpmPadeHelper < T , D > where T : ComplexField , D : DimMin < D > , DefaultAllocator : Allocator < D , D > + Allocator < DimMinimum < D , D > > , { use_exact_norm : bool , ident : OMatrix < T , D , D > , a : OMatrix < T , D , D > , a2 : Option < OMatrix < T , D , D > > , a4 : Option < OMatrix < T , D , D > > , a6 : Option < OMatrix < T , D , D > > , a8 : Option < OMatrix < T , D , D > > , a10 : Option < OMatrix < T , D , D > > , d4_exact : Option < T :: RealField > , d6_exact : Option < T :: RealField > , d8_exact : Option < T :: RealField > , d10_exact : Option < T :: RealField > , d4_approx : Option < T :: RealField > , d6_approx : Option < T :: RealField > , d8_approx : Option < T :: RealField > , d10_approx : Option < T :: RealField > , }
};
}
