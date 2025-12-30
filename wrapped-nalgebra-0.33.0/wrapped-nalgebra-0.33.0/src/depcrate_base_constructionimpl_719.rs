// Generated macro for impl_719 (impl)
macro_rules! Depcrate_base_constructionimpl_719 {
() => {
// Module: crate::base::construction
// Provides: {"impl_719"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T , R , C > Arbitrary for OMatrix < T , R , C > where R : Dim , C : Dim , T : Scalar + Arbitrary + Send , DefaultAllocator : Allocator < R , C > , Owned < T , R , C > : Clone + Send , { # [inline] fn arbitrary (g : & mut Gen) -> Self { let nrows = R :: try_to_usize () . unwrap_or (usize :: arbitrary (g) % 10) ; let ncols = C :: try_to_usize () . unwrap_or (usize :: arbitrary (g) % 10) ; Self :: from_fn_generic (R :: from_usize (nrows) , C :: from_usize (ncols) , | _ , _ | { T :: arbitrary (g) }) } }
};
}
