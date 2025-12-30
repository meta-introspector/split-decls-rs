// Generated macro for impl_1278 (impl)
macro_rules! Depcrate_geometry_pointimpl_1278 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1278"}
// Dependencies: {}
impl < T : Scalar + fmt :: Display , D : DimName > fmt :: Display for OPoint < T , D > where DefaultAllocator : Allocator < D > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{{") ? ; let mut it = self . coords . iter () ; < T as fmt :: Display > :: fmt (it . next () . unwrap () , f) ? ; for comp in it { write ! (f , ", ") ? ; < T as fmt :: Display > :: fmt (comp , f) ? ; } write ! (f , "}}") } }
};
}
