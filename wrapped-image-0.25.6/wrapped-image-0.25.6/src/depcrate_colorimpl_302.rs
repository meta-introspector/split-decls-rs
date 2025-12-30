// Generated macro for impl_302 (impl)
macro_rules! Depcrate_colorimpl_302 {
() => {
// Module: crate::color
// Provides: {"impl_302"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < LumaA < S > > for Rgba < T > where T : FromPrimitive < S > , { fn from_color (& mut self , gray : & LumaA < S >) { let rgba = & mut self . 0 ; let gray = & gray . 0 ; rgba [0] = T :: from_primitive (gray [0]) ; rgba [1] = T :: from_primitive (gray [0]) ; rgba [2] = T :: from_primitive (gray [0]) ; rgba [3] = T :: from_primitive (gray [1]) ; } }
};
}
