// Generated macro for inf (function)
macro_rules! Depcrateinf {
() => {
// Module: crate
// Provides: {"inf"}
// Dependencies: {}
# [doc = " Returns the infimum of `a` and `b`."] # [deprecated (note = "use the inherent method `Matrix::inf` instead")] # [inline] pub fn inf < T , R : Dim , C : Dim > (a : & OMatrix < T , R , C > , b : & OMatrix < T , R , C >) -> OMatrix < T , R , C > where T : Scalar + SimdPartialOrd , DefaultAllocator : Allocator < R , C > , { a . inf (b) }
};
}
