// Generated macro for sup (function)
macro_rules! Depcratesup {
() => {
// Module: crate
// Provides: {"sup"}
// Dependencies: {}
# [doc = " Returns the supremum of `a` and `b`."] # [deprecated (note = "use the inherent method `Matrix::sup` instead")] # [inline] pub fn sup < T , R : Dim , C : Dim > (a : & OMatrix < T , R , C > , b : & OMatrix < T , R , C >) -> OMatrix < T , R , C > where T : Scalar + SimdPartialOrd , DefaultAllocator : Allocator < R , C > , { a . sup (b) }
};
}
