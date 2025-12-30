// Generated macro for impl_2376 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2376 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2376"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > PartialEq for Similarity < T , R , D > where R : AbstractRotation < T , D > + PartialEq , { # [inline] fn eq (& self , right : & Self) -> bool { self . isometry == right . isometry && self . scaling == right . scaling } }
};
}
