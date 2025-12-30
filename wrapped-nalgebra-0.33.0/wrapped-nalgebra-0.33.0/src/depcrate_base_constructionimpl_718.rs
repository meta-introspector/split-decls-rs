// Generated macro for impl_718 (impl)
macro_rules! Depcrate_base_constructionimpl_718 {
() => {
// Module: crate::base::construction
// Provides: {"impl_718"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : Scalar , R : Dim , C : Dim > Distribution < OMatrix < T , R , C > > for Standard where DefaultAllocator : Allocator < R , C > , Standard : Distribution < T > , { # [inline] fn sample < G : Rng + ? Sized > (& self , rng : & mut G) -> OMatrix < T , R , C > { let nrows = R :: try_to_usize () . unwrap_or_else (| | rng . gen_range (0 .. 10)) ; let ncols = C :: try_to_usize () . unwrap_or_else (| | rng . gen_range (0 .. 10)) ; OMatrix :: from_fn_generic (R :: from_usize (nrows) , C :: from_usize (ncols) , | _ , _ | rng . gen ()) } }
};
}
