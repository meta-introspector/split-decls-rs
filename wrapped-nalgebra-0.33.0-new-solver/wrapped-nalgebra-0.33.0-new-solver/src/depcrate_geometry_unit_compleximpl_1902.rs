// Generated macro for impl_1902 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1902 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1902"}
// Dependencies: {}
impl < T : RealField + fmt :: Display > fmt :: Display for UnitComplex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "UnitComplex angle: {}" , self . angle ()) } }
};
}
