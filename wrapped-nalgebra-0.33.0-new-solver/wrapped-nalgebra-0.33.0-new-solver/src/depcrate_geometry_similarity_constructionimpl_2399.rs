// Generated macro for impl_2399 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2399 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2399"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : crate :: RealField , R , const D : usize > Distribution < Similarity < T , R , D > > for Standard where R : AbstractRotation < T , D > , Standard : Distribution < T > + Distribution < R > , { # [doc = " Generate an arbitrary random variate for testing purposes."] # [inline] fn sample < 'a , G : Rng + ? Sized > (& self , rng : & mut G) -> Similarity < T , R , D > { let mut s = rng . gen () ; while relative_eq ! (s , T :: zero ()) { s = rng . gen () } Similarity :: from_isometry (rng . gen () , s) } }
};
}
