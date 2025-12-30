// Generated macro for impl_303 (impl)
macro_rules! Depcrate_colorimpl_303 {
() => {
// Module: crate::color
// Provides: {"impl_303"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Luma < S > > for Rgba < T > where T : FromPrimitive < S > , { fn from_color (& mut self , gray : & Luma < S >) { let rgba = & mut self . 0 ; let gray = gray . 0 [0] ; rgba [0] = T :: from_primitive (gray) ; rgba [1] = T :: from_primitive (gray) ; rgba [2] = T :: from_primitive (gray) ; rgba [3] = T :: DEFAULT_MAX_VALUE ; } }
};
}
