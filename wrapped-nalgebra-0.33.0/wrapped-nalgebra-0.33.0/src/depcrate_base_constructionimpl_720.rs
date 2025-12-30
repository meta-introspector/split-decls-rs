// Generated macro for impl_720 (impl)
macro_rules! Depcrate_base_constructionimpl_720 {
() => {
// Module: crate::base::construction
// Provides: {"impl_720"}
// Dependencies: {}
# [cfg (feature = "rand")] impl < T : crate :: RealField , D : DimName > Distribution < Unit < OVector < T , D > > > for Standard where DefaultAllocator : Allocator < D > , rand_distr :: StandardNormal : Distribution < T > , { # [doc = " Generate a uniformly distributed random unit vector."] # [inline] fn sample < G : Rng + ? Sized > (& self , rng : & mut G) -> Unit < OVector < T , D > > { Unit :: new_normalize (OVector :: from_distribution_generic (D :: name () , Const :: < 1 > , & rand_distr :: StandardNormal , rng ,)) } }
};
}
