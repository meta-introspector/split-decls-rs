// Generated macro for impl_1276 (impl)
macro_rules! Depcrate_geometry_pointimpl_1276 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1276"}
// Dependencies: {}
impl < T : Scalar + PartialOrd , D : DimName > PartialOrd for OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . coords . partial_cmp (& other . coords) } # [inline] fn lt (& self , right : & Self) -> bool { self . coords . lt (& right . coords) } # [inline] fn le (& self , right : & Self) -> bool { self . coords . le (& right . coords) } # [inline] fn gt (& self , right : & Self) -> bool { self . coords . gt (& right . coords) } # [inline] fn ge (& self , right : & Self) -> bool { self . coords . ge (& right . coords) } }
};
}
