// Generated macro for solve_p_q (function)
macro_rules! Depcrate_linalg_expsolve_p_q {
() => {
// Module: crate::linalg::exp
// Provides: {"solve_p_q"}
// Dependencies: {}
fn solve_p_q < T , D > (u : OMatrix < T , D , D > , v : OMatrix < T , D , D >) -> OMatrix < T , D , D > where T : ComplexField , D : DimMin < D , Output = D > , DefaultAllocator : Allocator < D , D > + Allocator < DimMinimum < D , D > > , { let p = & u + & v ; let q = & v - & u ; q . lu () . solve (& p) . unwrap () }
};
}
