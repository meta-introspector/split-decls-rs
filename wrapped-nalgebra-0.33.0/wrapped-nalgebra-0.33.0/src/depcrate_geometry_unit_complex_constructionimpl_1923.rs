// Generated macro for impl_1923 (impl)
macro_rules! Depcrate_geometry_unit_complex_constructionimpl_1923 {
() => {
// Module: crate::geometry::unit_complex_construction
// Provides: {"impl_1923"}
// Dependencies: {}
# [cfg (feature = "rand")] impl < T : SimdRealField > Distribution < UnitComplex < T > > for Standard where T :: Element : SimdRealField , rand_distr :: UnitCircle : Distribution < [T ; 2] > , { # [doc = " Generate a uniformly distributed random `UnitComplex`."] # [inline] fn sample < 'a , R : Rng + ? Sized > (& self , rng : & mut R) -> UnitComplex < T > { let x = rng . sample (rand_distr :: UnitCircle) ; UnitComplex :: new_unchecked (Complex :: new (x [0] . clone () , x [1] . clone ())) } }
};
}
