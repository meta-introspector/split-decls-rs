// Generated macro for inf_sup (function)
macro_rules! Depcrateinf_sup {
() => {
// Module: crate
// Provides: {"inf_sup"}
// Dependencies: {}
# [doc = " Returns simultaneously the infimum and supremum of `a` and `b`."] # [deprecated (note = "use the inherent method `Matrix::inf_sup` instead")] # [inline] pub fn inf_sup < T , R : Dim , C : Dim > (a : & OMatrix < T , R , C > , b : & OMatrix < T , R , C > ,) -> (OMatrix < T , R , C > , OMatrix < T , R , C >) where T : Scalar + SimdPartialOrd , DefaultAllocator : Allocator < R , C > , { a . inf_sup (b) }
};
}
