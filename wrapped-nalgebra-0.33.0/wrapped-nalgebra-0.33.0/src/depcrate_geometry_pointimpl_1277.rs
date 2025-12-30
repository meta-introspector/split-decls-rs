// Generated macro for impl_1277 (impl)
macro_rules! Depcrate_geometry_pointimpl_1277 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1277"}
// Dependencies: {}
impl < T : Scalar + SimdPartialOrd , D : DimName > OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [doc = " Computes the infimum (aka. componentwise min) of two points."] # [inline] # [must_use] pub fn inf (& self , other : & Self) -> OPoint < T , D > { self . coords . inf (& other . coords) . into () } # [doc = " Computes the supremum (aka. componentwise max) of two points."] # [inline] # [must_use] pub fn sup (& self , other : & Self) -> OPoint < T , D > { self . coords . sup (& other . coords) . into () } # [doc = " Computes the (infimum, supremum) of two points."] # [inline] # [must_use] pub fn inf_sup (& self , other : & Self) -> (OPoint < T , D > , OPoint < T , D >) { let (inf , sup) = self . coords . inf_sup (& other . coords) ; (inf . into () , sup . into ()) } }
};
}
